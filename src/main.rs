mod core;
mod switch;
mod parser;
mod rpc;

use crate::core::{InputMethodMode, SupportLanguage};
use crate::parser::Parser;
use crate::rpc::*;
use crate::switch::Switcher;

use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{Duration, Instant};

/// 若长时间无客户端连接则退出（秒）
const IDLE_ACCEPT_TIMEOUT_SECS: u64 = 300;

fn main() {
    let mut server = Sever::new();
    let (port, listener) = server.init_listener();
    println!("{}", port);
    loop {
        // 当客户端失去连接时，等待重连
        let mut client = match server.accept_client(&listener) {
            Ok(client) => client,
            Err(_) => break,  // 超时结束监听
        };
        let cid = server.next_cid();
        match server.handle_client(cid, &mut client) {
            Ok(_) => break,  // 收到退出指令
            _ => continue,
        };
    }
}


struct Sever {
    switcher: Switcher,
    parser: Parser,
    current_cid: AtomicU16,
}
impl Sever {
    fn new() -> Sever {
        let switcher = match Switcher::new() {
            Ok(s) => s,
            Err(e) => {
                panic!("Switcher init failed: {}", e);
            }
        };
        let parser = Parser::new();
        Sever { switcher, parser, current_cid: AtomicU16::new(1) }
    }

    fn init_listener(&self) -> (u16, TcpListener) {
        match init_socket() {
            Ok((p, l)) => (p, l),
            Err(e) => { panic!("Not found available port! {e}") }
        }
    }

    fn accept_client(&self, listener: &TcpListener) -> Result<TcpStream, io::Error> {
        // 轮询监听，无连接睡眠，超时自动退出
        listener.set_nonblocking(true).expect("Set non-blocking failed!");
        let mut timeout = Duration::from_secs(IDLE_ACCEPT_TIMEOUT_SECS);
        let mut deadline = Instant::now() + timeout;
        loop {
            match accept_connect(listener) {
                Ok(stream) => {
                    stream.set_nonblocking(false).expect("Set blocking failed!");
                    listener.set_nonblocking(false).expect("Set blocking failed!");
                    return Ok(stream);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    if Instant::now() >= deadline {
                        eprintln!("Exiting server");
                        return Err(io::Error::new(io::ErrorKind::TimedOut, "accept timeout"))
                    }
                    std::thread::sleep(Duration::from_millis(50));
                    continue;
                }
                Err(_) => {
                    timeout = Duration::from_secs(IDLE_ACCEPT_TIMEOUT_SECS);
                    deadline = Instant::now() + timeout;
                    continue;
                }
            }
        }
    }

    fn handle_client(&mut self, cid: u16, client: &mut TcpStream) -> io::Result<()> {
        loop {
            let message = recv_message(client)?;
            let request = ClientRequest::from_json_message(message);
            let response = match request {
                Ok(req) => {
                    match req.command {
                        CommandMode::Analyze => self._analyze_switch(cid, req),
                        CommandMode::MethodOnly => self._method_switch(cid, req),
                        CommandMode::Switch => self._grammar_analysis(cid, req),
                        CommandMode::Exit => {
                            eprintln!("Exiting server");
                            return Ok(())
                        },
                    }
                },
                Err(err) => {
                    ClientResponse::new(
                        cid, false, Some(format!("Failed to analysis request! {err}")), None,
                    )
                }
            };
            send_message(client, response.to_json_message())?;
        }
    }

    fn next_cid(&self) -> u16 {
        let next = self.current_cid.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
        if next == 0 {
            self.next_cid()
        } else {
            next
        }
    }

    fn _grammar_analysis(&mut self, cid: u16, req: ClientRequest) -> ClientResponse {
        // Command::Analyze 请求响应

        let params = match req.params.to_analyze_params() {
            Ok(p) => p,
            Err(e) => return ClientResponse::new(cid, false, Some(e.to_string()), None),
        };

        let language = SupportLanguage::from_string(&params.language);
        if language.is_none() {
            return ClientResponse::new(req.cid, false, Some("Unsupported language!".to_string()), None);
        };
        // 更新语法树 并判断 cursor 是否在 comment 节点内部
        let language = language.unwrap();
        self.parser.add_language(language);
        self.parser.update_tree(language, &params.code);
        let grammar = GrammarMode::from_bool(
            self.parser.get_comments(language, &params.code).in_range(&params.cursor)
        );

        let res = AnalyzeResult { grammar };
        ClientResponse::new(cid, true, None, Some(CommandResult::from_analyze_result(res)))
    }

    fn _method_switch(&mut self, cid: u16, req: ClientRequest) -> ClientResponse {
        // 处理 Command::MethodOnly 请求响应

        let params = match req.params.to_method_only_params() {
            Ok(p) => p,
            Err(e) => return ClientResponse::new(cid, false, Some(e.to_string()), None),
        };
        let target_mode = match InputMethodMode::from_str(params.mode) {
            Ok(m) => m,
            Err(e) => return ClientResponse::new(cid, false, Some(e.to_string()), None),
        };
        let success = self.switcher.switch(target_mode);
        if success.is_err() {
            return ClientResponse::new(cid, false, Some(success.err().unwrap().to_string()), None);
        };

        let res = match self.switcher.query() {
            Ok(method) => MethodOnlyResult { method },
            Err(e) => return ClientResponse::new(cid, false, Some(e.to_string()), None),
        };
        ClientResponse::new(cid, true, None, Some(CommandResult::from_method_only_result(res)))
    }

    fn _analyze_switch(&mut self, cid: u16, request: ClientRequest) -> ClientResponse {
        // 处理命令：需要 language、code、cursor
        let params = match request.params.to_switch_params() {
            Ok(p) => p,
            Err(e) => return ClientResponse::new(cid, false, Some(e.to_string()), None),
        };
        let language = SupportLanguage::from_string(&params.language);
        if language.is_none() {
            return ClientResponse::new(request.cid, true, None, None);
        };

        let language = language.unwrap();
        // 更新语法树 并判断 cursor 是否在 comment 节点内部
        self.parser.add_language(language);
        self.parser.update_tree(language, &params.code);
        let comment = GrammarMode::from_bool(
            self.parser.get_comments(language, &params.code).in_range(&params.cursor)
        );
        // 根据 comment 决定是否切换输入法
        let switch = match comment {
            GrammarMode::Comment => { self.switcher.switch(InputMethodMode::Native) },
            GrammarMode::Code => { self.switcher.switch(InputMethodMode::English) }
        };
        let error = match switch {
            Ok(s) => {
                if s { None } else { Some("Switch input method failed".to_string()) }
            },
            Err(e) => Option::from(e.to_string())
        };
        let input_method = self.switcher.query().unwrap_or_else(|_| InputMethodMode::English);
        let res = SwitchResult { grammar: comment, method: input_method };
        ClientResponse::new(cid, true, error, Some(CommandResult::from_switch_result(res)))
    }
}
