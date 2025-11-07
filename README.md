# ECL (EthernosCommandLang)

## 项目概述

ECL是一个基于Rust开发的半过程式编程语言，主要用于教学和基本数学运算。该项目包含完整的编译器前端（词法分析、语法分析）和解释器实现。

### 核心特性
- **自定义表达式语法**：支持独特的`expr`关键字定义表达式函数
- **多种参数类型**：支持`l`, `r`, `l r`, `r1 r2`, `r1 argv`等参数模式
- **教学友好**：语法简洁，适合编程教学
- **无外部依赖**：纯Rust实现，仅使用标准库

### 技术栈
- **语言**：Rust (Edition 2024)
- **构建工具**：Cargo
- **依赖**：无外部依赖

## 项目结构

```
ECL/
├── src/                      # 源代码目录
│   ├── main.rs              # 程序入口，支持REPL和文件模式
│   ├── lexer.rs             # 词法分析器
│   ├── parser.rs            # 语法分析器
│   ├── interpreter.rs       # 解释器
│   ├── ast.rs               # 抽象语法树(AST)定义
│   ├── token.rs             # 词法单元(Token)定义
│   ├── error.rs             # 错误处理
│   └── repl.rs              # REPL交互式环境
├── *.ecl                    # ECL语言示例文件
├── Cargo.toml               # Rust项目配置
└── test_repl.bat           # Windows批处理测试脚本
```

## 构建与运行

### 基本命令

```bash
# 构建项目
cargo build

# 运行REPL模式
cargo run

# 执行ECL文件
cargo run filename.ecl

# 构建Release版本
cargo build --release
```

### 运行示例

```bash
# 运行Hello World示例
cargo run hello.ecl

# 运行函数测试
cargo run test_function_simple.ecl

# 运行乘法表示例
cargo run multiplication.ecl

# 运行演示示例
cargo run demo.ecl
```

### Windows测试

```bash
# 运行REPL测试脚本
test_repl.bat
```

## 语言特性

### 关键字
- `var` - 变量声明
- `const` - 常量声明
- `func` - 函数定义
- `expr` - 表达式函数定义
- `if`/`else` - 条件语句
- `for`/`in` - 循环语句
- `return` - 返回值
- `print` - 打印输出（不换行）
- `println` - 打印输出（换行）

### 表达式函数语法

ECL支持独特的表达式函数定义，允许中缀调用语法：

```ecl
// 定义表达式函数
expr x2(l a) {
    return a * 2;
}

// 调用方式1：中缀语法
print(16 x2);      // 输出: 32

// 调用方式2：括号语法（推荐）
print((16)x2);     // 输出: 32

// 多参数表达式
expr x(l a, r b) {
    return a * b;
}

print(2 x 16);     // 输出: 32
print((2)x(16));   // 输出: 32
```

### 数据类型
- **Number**：浮点数（内部使用f64）
- **String**：字符串
- **Range**：范围（如`1..5`）

### 控制流

```ecl
// if/else语句
if (a > b) {
    return a;
} else {
    return b;
}

// for循环（范围）
for i in 1..5 {
    println(i);
}
```

## 开发指南

### 代码规范
- 遵循Rust标准编码风格
- 使用`cargo fmt`格式化代码
- 使用`cargo clippy`检查代码质量

### 添加新功能
1. **词法分析**：在`lexer.rs`中添加新的Token类型
2. **语法分析**：在`parser.rs`中实现新的语法规则
3. **AST节点**：在`ast.rs`中定义新的AST节点类型
4. **解释执行**：在`interpreter.rs`中实现执行逻辑

### 调试技巧
- 使用`println!`在Rust代码中调试
- ECL代码中使用`print`和`println`输出中间结果
- 查看`target/debug`目录下的编译输出

## 示例文件说明

| 文件名 | 说明 |
|--------|------|
| `hello.ecl` | Hello World示例 |
| `demo.ecl` | 变量、循环、计算综合示例 |
| `test_function_simple.ecl` | 函数定义和调用示例 |
| `test_function.ecl` | 高级函数功能测试 |
| `multiplication.ecl` | 嵌套循环打印乘法表 |
| `format_test.ecl` | 格式化输出测试 |
| `speed.ecl` | 性能测试 |
| `simple_test.ecl` | 简单功能测试 |
| `repl_demo.ecl` | REPL演示 |
| `t.ecl` | 临时测试文件 |

## 常见问题

### REPL模式无法退出
在REPL中输入EOF（Ctrl+D on Linux/Mac, Ctrl+Z on Windows）或输入错误语法强制退出。

### 表达式函数调用失败
确保使用正确的语法：
- 中缀语法需要空格：`16 x2`
- 括号语法更可靠：`(16)x2`

### Windows路径问题
使用双反斜杠或正斜杠：`cargo run path\to\file.ecl` 或 `cargo run path/to/file.ecl`

## 高级特性

### 错误处理
项目实现了详细的错误处理机制：
- 词法错误提示
- 语法错误定位
- 运行时错误追踪

### 性能优化
- 使用HashMap存储变量和函数
- 输出缓冲机制
- 源码行信息追踪（便于错误定位）

## 待办事项
- [ ] 添加更多数学函数库
- [ ] 支持数组和字典类型
- [ ] 实现模块化系统
- [ ] 添加字符串处理函数
- [ ] 支持文件I/O操作
- [ ] 添加单元测试框架
