<div align="center">
<h3>LazyInputSwitcher</h3>
<P>一个 基于语法感知的本地输入法切换服务端，使用 Rust 编写</P>
</div>

🌍 [简体中文](./README.md)

---

### ✨ 项目简介

##### 本项目是一个 本地 TCP 服务端程序，核心目标是：

- ⏱️ 低延迟，平均响应时间 < 5 ms
- 🌲 基于 Tree-sitter 的语法分析
- 💬 精确判断光标是否位于注释区域
- ⌨️ 根据语法上下文自动切换系统输入法
- 🌐 通过 JSON 协议与客户端通信

该服务被设计为一个后端能力模块，即不提供 UI 也不维护多客户端状态

##### ⌨️ 输入法支持

| 平台      | 状态     | 说明                                          |
|---------|--------|---------------------------------------------|
| Windows | ✅ 已支持  | 支持系统输入法微软拼音，但由于windows安全限制，必须安装两种及以上语言的键盘布局 |
| Linux   | ✅ 已支持  | Fcitx5 输入法框架                                |
| macOS   | ⚠️ 未测试 | 代码存在，但无实机验证。 macOS 支持欢迎有环境的贡献者协助测试与完善。      |

### 🏗️ 架构与运行模型

- 单线程
- 单客户端
- 阻塞式 TCP 网络交互

##### 🚀 启动行为

为方便多服务器实例，由操作系统自动分配可用端口
仅在启动时通过 stdout 输出端口号
之后不再输出任何标准输出内容

```bash
25565
```

客户端应自行捕获该输出并建立 TCP 连接。

##### ⏱️ 生命周期管理

客户端应当负责服务端的生命周期管理
客户端负责启动本服务端，并在需要时向服务端发送 Exit 退出指令

##### 📡 通信协议

采用**json**格式作为通信文本，网络消息均为明文传输，未经加密

tcp消息结构为：8字节u64消息长度 + json utf8消息

客户端请求样式：

```
{
    cid: u16,
    // 客户端ID，用于标识客户端身份 首次连接至服务端时自动分配cid
    
    // Exit 时服务端将会结束自身的运行，服务端一段时间内无客户端连接也会自动退出
    // Switch 时 将会执行语法分析 与输入法自动切换
    // Analyze 时 仅执行 语法分析
    // MethodOnly 时 仅执行输入法切换
    command: Exit, Switcher, Analyze, MethodOnly
    
    /// 按照命令类型区分 Analyze 参数
    params: {
        code: String,
        // 原始代码
        
        // 代码类型,注意首字母大写
        // 名称应与 crate::core::SupportLanguage 枚举中保持一致
        language: String,
        // 光标位置 UTF-8 字节位置, 0基
        cursor: {
            row: usize,
            column: usize  // 行内字节偏移量
        }
    },
    /// MethodOnly 参数
    params: {
        mode: Native / English
        // 目标输入法, 首字母大写
    },
    /// Switch 参数
    params: {
        code: String,
        // 原始代码
        
        // 代码类型,注意首字母大写
        // 名称应与 crate::core::SupportLanguage 枚举中保持一致
        language: String,
        // 光标位置 UTF-8 字节位置, 0基
        cursor: {
            row: usize,
            column: usize  // 行内字节偏移量
        }
    },
    /// Exit 参数
    params: {
        // 无参数, 空的 一对花括号
    }
}
```

服务端回复样式：

```
{
    cid: u16,                 // 服务端确认的客户端 ID（由服务端分配）
    success: bool,            // 请求是否成功（true 表示操作成功）
    error: Null / String      // 当 success = false 时, 此字段包含执行错误原因 成功则为 Null
    result: Null / {}         // 当 success = true 时，此字段包含执行结果 不成功则为 Null

    /// Analyze 请求结果
    result: {
        grammar: Comment / Code
    }

    /// ModeOnly 请求结果
    result: {
        method: Native / English
    }

    /// Switch 请求结果
    result: {
        grammar : Comment / Code,
        method: Native / English,
    }

    /// Exit 无请求结果, 服务器将断开网络连接之后结束自身
}
```

### 🌲 语法分析设计

使用 Tree-sitter Query 提取注释节点，支持多语言语法树，同时方便拓展对更多编程语言的支持。

##### 🎯 注释区间判断策略（核心逻辑）

注释判断遵循以下原则：

1. 严格的左开右闭区间判断
2. 当光标位于注释结束位置时，检查注释结束后至行尾的字符。若仅包含空白或换行，则仍判定为注释内；

### 🧪 测试

项目包含独立的 tests 模块，用于验证： 请求 / 响应序列化与解析； 注释区间判断边界行为

### 🔐 隐私与安全

###

- ❌ 不连接至互联网
- ❌ 不收集用户数据
- ❌ 不记录或缓存源代码内容
- ✅ 所有逻辑仅在本地进程内完成

基于本项目构建的neovim输入法插件：[lazyime.nvim](https://github.com/StellarDeca/lazyime.nvim)

### 📦 构建与运行

通过 Github CI 自动构建 Release。 若预编译的 release 可执行文件无法在目标环境运行，
请克隆本仓库至本地，安装rust环境并运行本地测试与构建命令：

```bash
cargo test
cargo build --release
```
---

### 受支持的编程语言

当前服务器端已适配并支持以下编程语言：

|---------------------|
| **C** |
| **C++** |
| **C#** |
| **Java** |
| **JavaScript** |
| **TypeScript** |
| **Kotlin** |
| **Python** |
| **Rust** |
| **Lua** |
| **Go** |
| **Bash** |
| **SQL** |
| **PHP** |

> ⚠️ 说明  
> 服务器端仅对上述语言提供完整的 Tree-sitter 语法解析与注释节点提取能力。  
> 若某语言未在列表中（例如 Markdown），则表示该语言尚未完成服务器端适配。

---

### 添加更多受支持的编程语言

本项目基于 **Tree-sitter** 构建，对新增编程语言的支持流程清晰、扩展成本较低。  
以下步骤描述了如何在服务器端完整地添加一种新的编程语言支持。

##### 1. 添加 Tree-sitter 语言依赖

在 `Cargo.toml` 中引入目标语言对应的 Tree-sitter crate。
```toml
[dependencies]
tree-sitter-rust = "0.24.0"
```
确保该 crate 对外暴露 `LANGUAGE` 常量。

##### 2. 编写 Tree-sitter Query（注释节点）

在以下目录中新增对应语言的 Tree-sitter Query 文件：

```
server/src/static/TreeSitterQuery/
```

- 文件名必须与 `SupportLanguage` 枚举名称一致
- 全部使用 **小写字母**
- 文件扩展名为 `.scm`

如 Rust Query 为 rust.scm

Query 的目标是 **精确匹配该语言的注释节点**，并统一使用 `@comment` 作为捕获节点名称。

##### 3. 扩展 SupportLanguage 枚举

在 `src/core/lib.rs` 中：

1. 向 `SupportLanguage` 枚举中新增目标语言
2. 更新 `from_string` 方法，使其能够将字符串语言标识映射到对应枚举

```rust
pub enum SupportLanguage {
    Rust,
}
```

```rust
impl SupportLanguage {
    pub fn from_string(lang: &str) -> Option<Self> {
        let lang = s.to_lowercase();
        if lang == "rust" {
            Some(SupportLanguage::Rust)
        } else {
            None
        }
    }
}
```

### 4. 在 Parser Adapter 中注册语言

在 `src/parser/adapter.rs` 的 `Adapter::new` 方法中：

1. 引入目标语言的 `LANGUAGE`
2. 将其注册到支持语言的 `HashMap` 中

```rust
impl Adapter {
    pub(super) fn new() -> Adapter {
        use tree_sitter_rust::LANGUAGE as rust_;
        let mut language: HashMap<SupportLanguage, Language> = HashMap::new();
        language.insert(SupportLanguage::Rust, rust_.into());
        Adapter { language }
    }
}
```

### 5. 验证

完成以上步骤后：

- 服务器即可识别并解析新语言
- 对应的注释节点可被正确提取
- 条件允许可在 tests/parser_tests.rs 中添加测试
- 客户端无需额外修改即可自动支持该语言，只需要更新本项目的可执行文件

构建完成后，运行生成的可执行文件即可启动服务。

### 🤝 贡献

欢迎提交 Issue 与 Pull Request

- 新的输入法实现
- 新编程语言支持
- 协议与架构改进建议等

### 💖 贡献者
<div align="center">
    <a href="https://github.com/StellarDeca/LazyInputSwitcher/graphs/contributors">
        <img src="https://contrib.rocks/image?repo=StellarDeca/LazyInputSwitcher"  alt="Authors"/>
    </a>
</div>


### 📄 License

**GNU Affero General Public License v3.0 (AGPL-3.0)**

**若你在网络服务中使用、修改并对外提供该程序， 你有义务向使用者提供修改后的完整源代码。**
