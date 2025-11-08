# 安装与配置

## 系统要求

ECL 可以在任何支持 Rust 的系统上运行，包括：
- Windows 7 及以上版本
- macOS 10.13 及以上版本
- Linux (支持 x86_64 架构)

## 安装 Rust

ECL 使用 Rust 编写，因此需要先安装 Rust 工具链：

### Windows
```bash
# 下载并安装 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

或者访问 https://www.rust-lang.org/tools/install 下载安装程序。

### macOS/Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后，重启终端或运行：
```bash
source ~/.cargo/env
```

验证 Rust 安装：
```bash
rustc --version
cargo --version
```

## 获取 ECL 源代码

您可以从 GitHub 仓库克隆 ECL 源代码：

```bash
git clone <repository-url>
cd ECL
```

或者直接下载源代码压缩包并解压。

## 构建 ECL

在 ECL 项目目录中运行以下命令构建项目：

```bash
# 构建项目（调试版本）
cargo build

# 构建项目（发布版本）
cargo build --release
```

构建成功后，可执行文件位于 `target/debug/ecl` 或 `target/release/ecl`。

## 验证安装

运行以下命令验证 ECL 是否正确安装：

```bash
# 运行 REPL 模式
cargo run

# 或者执行示例文件
cargo run tests/hello.ecl
```

如果看到 "Hello, World!" 输出，则安装成功。

## 开发环境配置

### VS Code 配置
如果您使用 VS Code，建议安装以下扩展：
- Rust (rls) - Rust 语言支持
- rust-analyzer - Rust 语言服务器
- CodeLLDB - 调试器支持

### 项目配置文件
ECL 项目包含以下重要配置文件：

- `Cargo.toml` - Rust 项目配置文件，定义依赖和构建选项
- `Cargo.lock` - 锁定依赖版本
- `src/` - 源代码目录
- `tests/` - 测试文件目录

## 环境变量

ECL 不需要设置特殊的环境变量。但为了方便使用，您可以将构建后的可执行文件添加到系统 PATH 中：

```bash
# Linux/macOS
export PATH="$PATH:/path/to/ecl/target/release"

# Windows
set PATH=%PATH%;C:\path\to\ecl\target\release
```

## 故障排除

### 常见问题

1. **构建失败**
   - 确保 Rust 版本是最新的稳定版
   - 运行 `rustup update` 更新 Rust
   - 检查是否有缺失的系统依赖

2. **权限问题**
   - 在 Linux/macOS 上可能需要使用 `sudo` 安装
   - 确保对项目目录有读写权限

3. **路径问题**
   - 确保项目路径不包含特殊字符或空格
   - 在 Windows 上使用正斜杠或双反斜杠

### 获取帮助

如果遇到安装问题，可以通过以下方式获取帮助：
- 查看 GitHub 仓库的 Issues 页面
- 运行 `cargo run -- --help` 查看命令行选项
- 检查 `IFLOW.md` 文件获取更多技术细节