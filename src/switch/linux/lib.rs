use rust_embed::RustEmbed;
use std::error::Error;
use std::process::Command;

/// 打包shell脚本代码
/// shell代码加载与运行
#[derive(RustEmbed)]
#[folder = r"src/static/LinuxMethodShell/"]
pub(super) struct StaticLinuxMethodShell;
impl StaticLinuxMethodShell {
    /// 加载shell代码
    pub(super) fn get_script(name: &str) -> String {
        let file = StaticLinuxMethodShell::get(&format!("{}.sh", name)).unwrap();
        std::str::from_utf8(&file.data).unwrap().to_string()
    }

    /// 运行shell代码并获取结果
    pub(super) fn run_script(name: &str, args: Option<&[&str]>) -> Result<String, Box<dyn Error>> {
        // 将script交给bash解释器去执行，规避了权限问题与临时文件的问题
        let script = StaticLinuxMethodShell::get_script(name);
        let mut command = Command::new("bash");
        command.arg("-c").arg(script).arg("bash_script.sh");   // 注意 $0 占位符
        if let Some(args) = args {
            for arg in args.iter() {
                command.arg(arg);
            }
        }
        let output = command.output()?;

        // 处理执行结果
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!("Script failed: {}", String::from_utf8_lossy(&output.stderr)).into())
        }
    }
}

pub(super) enum SupportMethod {
    Fcitx5,
    Ibus,
}
impl SupportMethod {
    fn check_input_method() -> Option<SupportMethod> {
        let method = StaticLinuxMethodShell::run_script("check", None);
        match method {
            Ok(output) => {
                if output == "Fcitx5" {
                    Some(SupportMethod::Fcitx5)
                } else if output == "Ibus" {
                    Some(SupportMethod::Ibus)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
