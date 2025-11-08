# 快速开始

本指南将帮助您快速上手 ECL 编程语言。

## 启动 REPL

最简单的开始方式是使用 ECL 的交互式环境 (REPL)：

```bash
cargo run
```

您将看到类似以下的欢迎信息：
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

## 第一个程序

在 REPL 中输入以下代码：

```ecl
print("Hello, ECL!");
```

您应该看到输出：
```
Hello, ECL!
```

## 变量和基本运算

让我们尝试使用变量和基本运算：

```ecl
var x = 10;
var y = 20;
var sum = x + y;
print("Sum is: ");
print(sum);
```

输出：
```
Sum is: 30
```

## 控制流

### 条件语句

```ecl
var age = 18;
if (age >= 18) {
    println("You are an adult");
} else {
    println("You are a minor");
}
```

### 循环语句

```ecl
// 使用范围循环
for i in 1..5 {
    print("Number: ");
    print(i);
    println("");
}

// 使用 while 循环
var counter = 0;
while (counter < 3) {
    print("Counter: ");
    print(counter);
    println("");
    counter = counter + 1;
}
```

## 函数定义

### 普通函数

```ecl
func add(a, b) {
    return a + b;
}

var result = add(5, 3);
print("5 + 3 = ");
print(result);
```

### 表达式函数

```ecl
expr multiply(l a, r b) {
    return a * b;
}

// 中缀调用语法
var product = (4)multiply(5);
print("4 * 5 = ");
print(product);
```

## 数据结构

### 数组

```ecl
// 创建整型数组
var <int>arr[5] = {0};
arr[0] = 10;
arr[1] = 20;
print("First element: ");
print(arr[0]);
```

### 列表

```ecl
// 创建列表
var lst = {1, "hello", true};
print("List element: ");
print(lst[1]);
```

## 类型转换

```ecl
var num = 123;
var str = <str>num;
print("Number as string: ");
print(str);
```

## 多行输入

对于复杂的代码块，可以使用多行输入模式：

```ecl
{{  // 开始多行输入
var sum = 0;
for i in 1..10 {
    sum = sum + i;
}
print("Sum of 1 to 10: ");
print(sum);
}}  // 结束多行输入并执行
```

## 保存和运行文件

创建一个名为 `hello.ecl` 的文件：

```ecl
// hello.ecl
println("Welcome to ECL!");
var name = "Alice";
print("Hello, ");
print(name);
println("!");

// 计算平方
expr square(l x) {
    return x * x;
}

var num = 5;
var result = (num)square;
print(num);
print(" squared is ");
print(result);
println("");
```

然后运行文件：

```bash
cargo run hello.ecl
```

## REPL 命令

REPL 环境提供了一些有用的命令：

- `help` - 显示帮助信息
- `history` - 显示命令历史
- `clear` - 清屏
- `exit` 或 `quit` - 退出 REPL
- `{{` - 开始多行输入模式
- `}}` - 结束多行输入模式并执行

## 下一步

现在您已经掌握了 ECL 的基本用法，可以：

1. 查看 [语法参考](./syntax.md) 了解更多语法规则
2. 学习 [数据类型](./types.md) 系统
3. 探索 [函数](./functions.md) 的高级用法
4. 尝试 [示例代码](./examples.md) 中的更多例子

继续学习，您将能够编写更复杂的 ECL 程序！