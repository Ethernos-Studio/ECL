# ECL (EthernosCommandLang) - iFlow CLI 项目指南

## 项目概述

ECL是一个基于Rust开发的半过程式编程语言，主要用于教学和基本数学运算。该项目包含完整的编译器前端（词法分析、语法分析）和解释器实现。

### 核心特性
- **自定义表达式语法**：支持独特的`expr`关键字定义表达式函数
- **多种参数类型**：支持`l`, `r`, `l r`, `r1 r2`, `r1 argv`等参数模式
- **教学友好**：语法简洁，适合编程教学
- **无外部依赖**：纯Rust实现，仅使用标准库
- **类型系统**：支持静态类型声明和类型转换
- **数据结构**：支持数组和列表数据结构
- **丰富语法**：支持变量、函数、控制流、输入输出等特性

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
├── tests/                   # 测试文件目录
│   ├── *.ecl                # ECL语言示例文件
│   └── debug_lexer.rs       # 词法分析器调试工具
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

### 调试模式

```bash
# 调试词法分析器
cargo run -- --debug-lexer "expression to analyze"

# 调试词法分析器（文件模式）
cargo run -- --debug-lexer filename.ecl

# 显示版本信息
cargo run -- --version
```

### 运行示例

```bash
# 运行Hello World示例
cargo run tests/hello.ecl

# 运行函数测试
cargo run tests/test_function_simple.ecl

# 运行乘法表示例
cargo run tests/multiplication.ecl

# 运行演示示例
cargo run tests/demo.ecl

# 运行数组和列表测试
cargo run tests/test_array_list.ecl
```

### Windows测试

```bash
# 运行REPL测试脚本
test_repl.bat
```

## 语言特性

### 关键字
- `var` - 变量声明
- `const` - 常量声明（保留关键字）
- `func` - 函数定义
- `expr` - 表达式函数定义
- `if`/`else` - 条件语句
- `for`/`in` - 循环语句
- `while` - while循环语句
- `return` - 返回值
- `print` - 打印输出（不换行）
- `println` - 打印输出（换行）
- `input` - 输入语句
- `true`/`false` - 布尔值

### 数据类型
- **int**：整数类型
- **float**：单精度浮点数类型
- **double**：双精度浮点数类型
- **str**：字符串类型
- **bool**：布尔类型
- **array**：固定长度同类型数组
- **list**：动态长度异类型列表

### 变量声明

```ecl
// 基本变量声明
var x = 10;
var name = "ECL";

// 带类型声明的变量
var <int>age = 25;
var <str>message = "Hello";

// 数组声明
var <int>arr[5] = {0};  // 长度为5的整型数组，初始化为0
var <int>arr2[3] = {1, 2, 3};  // 长度为3的整型数组，指定初始值

// 列表声明
var lst = [];  // 空列表
var lst2 = {1, "hello", true};  // 带初始值的列表
```

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
expr multiply(l a, r b) {
    return a * b;
}

print(2 multiply 16);     // 输出: 32
print((2)multiply(16));   // 输出: 32
```

### 函数定义

```ecl
// 普通函数
func add(a, b) {
    return a + b;
}

// 表达式函数（支持中缀调用）
expr subtract(l a, r b) {
    return a - b;
}

// 调用函数
var result = add(10, 5);
var diff = (10)subtract(5);
```

### 控制流

```ecl
// if/else语句
if (a > b) {
    print("a is greater");
} else {
    print("b is greater or equal");
}

// for循环（范围）
for i in 1..5 {
    println(i);
}

// for循环（表达式范围）
var limit = 10;
for i in 1..(limit/2) {
    println(i);
}

// while循环
var counter = 0;
while (counter < 5) {
    println(counter);
    counter = counter + 1;
}
```

### 数组和列表操作

```ecl
// 数组声明和操作
var <int>arr[5] = {0};
arr[0] = 10;  // 数组元素赋值
print(arr[0]);  // 访问数组元素

// 列表声明和操作
var lst = {1, "hello", true};
lst[0] = 100;  // 列表元素赋值
print(lst[0]);  // 访问列表元素
```

### 类型转换

```ecl
// 显式类型转换
var num = 123;
var str = <str>num;  // 转换为字符串
var flt = <float>num;  // 转换为浮点数
```

### 输入输出

```ecl
// 输出
print("Hello");
println("World");

// 输入
input("Enter your name: ", name);
println("Hello, " + name);
```

## REPL交互环境

ECL提供了一个功能丰富的REPL环境，支持以下特性：

```bash
# 启动REPL
cargo run

# REPL特性：
# - 自动补全未闭合的括号和大括号
# - 多行输入模式（使用{{开始，}}结束）
# - 命令历史记录
# - 内置帮助系统
```

### REPL命令
- `help` - 显示帮助信息
- `history` - 显示命令历史
- `clear` - 清屏
- `exit`/`quit` - 退出REPL
- `{{` - 开始多行输入模式
- `}}` - 结束多行输入模式并执行

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
5. **错误处理**：在`error.rs`中添加适当的错误消息

### 调试技巧
- 使用`--debug-lexer`参数调试词法分析器
- 在Rust代码中使用`println!`进行调试
- ECL代码中使用`print`和`println`输出中间结果
- 查看`target/debug`目录下的编译输出

## 示例文件说明

| 文件名 | 说明 |
|--------|------|
| `hello.ecl` | Hello World示例 |
| `demo.ecl` | 变量、循环、计算综合示例 |
| `multiplication.ecl` | 嵌套循环打印乘法表 |
| `format_test.ecl` | 格式化输出测试 |
| `speed.ecl` | 性能测试 |
| `simple_test.ecl` | 简单功能测试 |
| `repl_demo.ecl` | REPL演示 |
| `test_array_list.ecl` | 数组和列表功能测试 |
| `simple_type_test.ecl` | 类型转换测试 |
| `t.ecl` | 临时测试文件 |

## 常见问题

### REPL模式无法退出
在REPL中输入`exit`或`quit`命令退出，或输入EOF（Ctrl+D on Linux/Mac, Ctrl+Z on Windows）。

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
- 友好的错误消息和建议

### 性能优化
- 使用HashMap存储变量和函数
- 输出缓冲机制
- 源码行信息追踪（便于错误定位）

### 类型系统
- 静态类型声明（`<type>variable`）
- 显式类型转换（`<type>value`）
- 运行时类型检查
- 数组和列表类型支持

## 待办事项
- [ ] 添加更多数学函数库
- [ ] 支持模块化系统
- [ ] 添加字符串处理函数
- [ ] 支持文件I/O操作
- [ ] 添加单元测试框架
- [ ] 实现更复杂的类型系统
- [ ] 添加标准库支持