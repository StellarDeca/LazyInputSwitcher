//! 网络通信模块 服务端与客户端一一对应 使用同步装后台完成，不引入异步
//!
//! 解决 tcp 协议粘包问题
//! 对收发消息格式做出规定:
//!      [u64 message size][json message]
//!

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

/// 让系统分配可用端口 并返回端口 与 socket
pub(crate) fn init_socket() -> io::Result<(u16, TcpListener)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let port = addr.port();
    Ok((port, listener))
}

/// 等待客户端创建连接
/// return: 客户端连接 socket
pub(crate) fn accept_connect(listener: &TcpListener) -> io::Result<TcpStream> {
    let client_socket = listener.accept()?;
    Ok(client_socket.0)
}

/// 接受客户端消息并转换为 utf-8 字符串
pub(crate) fn recv_message(client: &mut TcpStream) -> io::Result<String> {
    // 读取 消息长度
    let mut len_buf = [0u8; 8];
    client.read_exact(&mut len_buf)?;
    let len = u64::from_be_bytes(len_buf) as usize;
    println!("{}", len);
    // 读取消息
    let mut buffer = vec![0u8; len];
    client.read_exact(&mut buffer)?;
    match std::str::from_utf8(&buffer) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
}

/// 向客户端发送消息
/// TCP底层有重发机制，这里不再实现
pub(crate) fn send_message(client: &mut TcpStream, message: String) -> io::Result<()> {
    // 使用 write_all 确保将整个 buffer 发送出去
    let len = (message.len() as u64).to_be_bytes();
    let mut buffer = Vec::with_capacity(8 + message.len());
    buffer.extend_from_slice(&len);
    buffer.extend(message.as_bytes());
    client.write_all(&buffer)?;
    Ok(())
}
