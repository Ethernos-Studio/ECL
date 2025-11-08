# REPL 环境

REPL (Read-Eval-Print Loop) 是 ECL 的交互式环境，允许您实时输入和执行代码。

## 启动 REPL

### 基本启动

```bash
# 在项目根目录运行
cargo run

# 或者构建后运行
cargo build
./target/debug/ecl
```

启动后您将看到：

```
╔═══════════════════════════════════════╗
║     ECL (ECL Command Language)       ║
║              REPL                    ║
╚═══════════════════════════════════════╝

💡 Type 'help' for help, 'exit' to quit
📝 Use '{{' to start multiline input, '}}' to end
🔍 Auto-completion: parentheses and braces are balanced

ecl> 
```

## 基本使用

### 单行命令

在 `ecl>` 提示符下直接输入 ECL 代码：

```ecl
ecl> print("Hello, World!");
Hello, World!
ecl> var x = 10;
ecl> print(x);
10
ecl> var result = x * 2;
ecl> print(result);
20
```

### 多行输入

对于复杂的代码块，使用多行输入模式：

```ecl
ecl> {{
ecl> var sum = 0;
ecl> for i in 1..5 {
ecl>     sum = sum + i;
ecl> }
ecl> print("Sum: " + sum);
ecl> }}
📝 Executing: var sum = 0;
for i in 1..5 {
    sum = sum + i;
}
print("Sum: " + sum);

─────────────────────────────────
Sum: 10
─────────────────────────────────
```

## REPL 命令

### 内置命令

- `help` - 显示帮助信息
- `history` - 显示命令历史
- `clear` - 清屏
- `exit` 或 `quit` - 退出 REPL

#### help 命令

```ecl
ecl> help
```

显示详细帮助信息，包括：
- REPL 命令列表
- 语言特性说明
- 使用示例

#### history 命令

```ecl
ecl> history
```

显示您在当前会话中输入的命令历史：

```
📋 Command History:
    1: var x = 10;
    2: print(x);
    3: var y = 20;
    4: var sum = x + y;
    5: print(sum);
    6: history
```

#### clear 命令

```ecl
ecl> clear
```

清空屏幕内容，返回到干净的 REPL 提示符。

#### exit/quit 命令

```ecl
ecl> exit
# 或
ecl> quit
```

退出 REPL 环境并返回到系统命令行。

## 特殊功能

### 自动补全

REPL 会自动补全未闭合的括号和大括号：

```ecl
ecl> print("Hello
📝 Executing: print("Hello")
Hello
```

输入 `print("Hello` 按回车后，REPL 会自动补全到 `print("Hello")`。

### 语法高亮和错误提示

REPL 提供即时的语法检查：

```ecl
ecl> var x = ;  // 输入错误语法
Syntax error: unexpected semicolon, please check if the previous statement is complete
  --> <repl>:1:9
  |
1 | var x = ;
  |         ^
  = help: syntax is `var name = value`
  = example: var name = "Alice"
```

## 实时执行

所有在 REPL 中输入的代码都会立即执行：

```ecl
ecl> var greeting = "Welcome to ECL!";
ecl> print(greeting);
Welcome to ECL!

ecl> func square(x) {
...>     return x * x;
...> }
ecl> var result = square(5);
ecl> print("5 squared is: " + result);
5 squared is: 25
```

## 变量持久性

在 REPL 会话中定义的变量在整个会话期间都可用：

```ecl
ecl> var counter = 0;
ecl> counter = counter + 1;
ecl> print("Counter: " + counter);
Counter: 1
ecl> counter = counter * 2;
ecl> print("Counter: " + counter);
Counter: 2
```

## 函数定义

可以在 REPL 中定义和使用函数：

```ecl
ecl> func factorial(n) {
...>     if (n <= 1) {
...>         return 1;
...>     }
...>     return n * factorial(n - 1);
...> }
ecl> print("5! = " + factorial(5));
5! = 120
```

## 表达式函数

同样支持表达式函数定义和使用：

```ecl
ecl> expr power(l base, r exp) {
...>     var result = 1;
...>     for i in 1..(exp+1) {
...>         result = result * base;
...>     }
...>     return result;
...> }
ecl> print("2^3 = " + (2)power(3));
2^3 = 8
```

## 调试功能

### 查看当前环境

虽然 REPL 没有内置的命令来列出所有变量，但您可以编写简单的调试代码：

```ecl
ecl> var debug = true;
ecl> if (debug) {
...>     print("Debug mode enabled");
...>     print("Current counter: " + counter);
...> }
Debug mode enabled
Current counter: 2
```

### 逐步执行

REPL 非常适合逐步执行代码和验证结果：

```ecl
ecl> var x = 10;
ecl> var y = 20;
ecl> var step1 = x + y;
ecl> print("Step 1: " + step1);
Step 1: 30
ecl> var step2 = step1 * 2;
ecl> print("Step 2: " + step2);
Step 2: 60
```

## 错误处理和调试

### 语法错误

当输入错误的语法时，REPL 会显示详细的错误信息：

```ecl
ecl> var x = 10
ecl> var y = 20;  // 可能缺少分号导致错误
```

### 运行时错误

```ecl
ecl> var arr[3] = {1, 2, 3};
ecl> print(arr[5]);  // 数组越界
Index out of bounds: index 5 is out of range for array of length 3
  --> <repl>:1:7
  |
1 | print(arr[5]);
  |       ^^^^^^^
  = help: make sure the index is within the valid range [0, length)
  = example: if arr has 5 elements, valid indices are 0, 1, 2, 3, 4
```

## 高级使用技巧

### 快速测试代码片段

REPL 非常适合快速测试代码片段：

```ecl
ecl> var testArray[5] = {0};
ecl> for i in 0..5 {
...>     testArray[i] = i * i;
...> }
ecl> for i in 0..5 {
...>     print("testArray[" + i + "] = " + testArray[i] + "\n");
...> }
testArray[0] = 0
testArray[1] = 1
testArray[2] = 4
testArray[3] = 9
testArray[4] = 16
```

### 演示语言特性

```ecl
ecl> // 演示类型转换
ecl> var num = 123;
ecl> var str = <str>num;
ecl> print("Number: " + num + ", String: " + str);
Number: 123, String: 123
```

### 交互式学习

REPL 是学习 ECL 语言特性的理想环境：

```ecl
ecl> // 尝试不同的循环
ecl> var sum = 0;
ecl> for i in 1..11 {
...>     sum = sum + i;
...> }
ecl> print("Sum of 1 to 10: " + sum);
Sum of 1 to 10: 55

ecl> // 尝试 while 循环
ecl> var count = 0;
ecl> while (count < 5) {
...>     print("Count: " + count + "\n");
...>     count = count + 1;
...> }
Count: 0
Count: 1
Count: 2
Count: 3
Count: 4
```

## 最佳实践

### 1. 代码组织

在多行输入中保持良好的缩进：

```ecl
ecl> {{
ecl> // 良好的缩进
ecl> if (condition) {
ecl>     for i in 0..5 {
ecl>         // 嵌套缩进
ecl>         process(i);
ecl>     }
ecl> }
ecl> }}
```

### 2. 逐步构建

对于复杂代码，逐步构建和测试：

```ecl
ecl> // 先测试基础功能
ecl> func helper(x) {
...>     return x * 2;
...> }
ecl> print(helper(5));  // 测试
ecl> // 然后构建更复杂的函数
ecl> func mainFunc(x) {
...>     var temp = helper(x);
...>     return temp + 10;
...> }
```

### 3. 使用注释

在复杂的 REPL 会话中使用注释：

```ecl
ecl> // 设置测试数据
ecl> var testValues = {1, 2, 3, 4, 5};
ecl> // 计算平方
ecl> var squares = {};
ecl> // 注意：当前ECL语法可能需要不同方式来创建数组
```

### 4. 保存有用的代码

对于在 REPL 中开发的有用代码，记得保存到文件中：

```ecl
// 在 REPL 中测试完成后
// 将代码复制到 .ecl 文件中保存
```

## 故障排除

### 1. 无法退出 REPL

如果 `exit` 或 `quit` 不起作用：

- 尝试使用 Ctrl+C 组合键
- 或者使用 Ctrl+D (EOF) 在 Linux/macOS 上
- 在 Windows 上可能需要 Ctrl+Z

### 2. 奇怪的自动补全行为

如果遇到意外的自动补全，可以：

- 确保括号和大括号正确匹配
- 使用 `clear` 命令重置显示
- 重新启动 REPL

### 3. 历史记录问题

如果历史记录出现问题：

- 新的会话不会保留之前的命令历史
- 历史记录只在当前会话中有效

## 与脚本执行的区别

REPL 与直接运行 ECL 脚本文件有几个关键区别：

| 特性 | REPL | 脚本文件 |
|------|------|----------|
| 交互性 | 高 | 低 |
| 即时反馈 | 是 | 否 |
| 错误处理 | 单行错误不影响后续 | 整个脚本停止 |
| 变量持久性 | 会话期间持久 | 仅在执行期间 |
| 函数定义 | 可以交互式定义 | 在脚本中定义 |

## 总结

ECL REPL 是一个强大的交互式开发环境，提供了：

- **即时执行**：代码输入后立即执行
- **多行支持**：复杂代码块的输入
- **命令历史**：查看和重用先前命令
- **自动补全**：括号和大括号自动补全
- **错误提示**：详细的错误信息和建议
- **交互式学习**：适合学习和测试语言特性

通过熟练使用 REPL，您可以快速实验 ECL 代码、调试问题，并加深对语言特性的理解。