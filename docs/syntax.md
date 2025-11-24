# 基本语法

ECL 语言具有简洁明了的语法结构，易于学习和使用。

## 程序结构

ECL 程序由一系列语句组成，每个语句以分号结尾（可选）：

```ecl
// 单行注释
/* 多行注释 */

// 语句可以有分号
var x = 10;

// 语句也可以没有分号
var y = 20
```

## 标识符

标识符用于命名变量、函数等，必须遵循以下规则：
- 以字母或下划线开头
- 后跟字母、数字或下划线
- 区分大小写

```ecl
// 有效的标识符
var name = "ECL";
var _private = 100;
var variable1 = 200;

// 无效的标识符
// var 1variable = 10;  // 不能以数字开头
// var my-variable = 20;  // 不能包含连字符
```

## 字面量

### 数字字面量

ECL 支持整数和浮点数字面量：

```ecl
// 整数字面量
var integer = 42;
var negative = -10;

// 浮点数字面量
var floatNum = 3.14;
var scientific = 1.5e-3;
```

### 字符串字面量

字符串使用双引号包围，支持反斜杠转义序列：

```ecl
// 基本字符串
var greeting = "Hello, World!";

// 转义序列
var newline = "Line 1\nLine 2";        // 换行符
var tab = "Name\tAge\tCity";           // 制表符
var quote = "She said \"Hello\"";      // 双引号
var backslash = "Path: C:\\Users";     // 反斜杠
var singleQuote = "It\'s working";     // 单引号

// Unicode转义
var emoji = "Smile: \u{1F600}";        // Unicode表情
var chinese = "中文: \u{4E2D}\u{6587}"; // Unicode中文字符
var ascii = "ABC: \u0041\u0042\u0043"; // Unicode ASCII字符
```

### 布尔字面量

```ecl
var isTrue = true;
var isFalse = false;
```

## 运算符

### 算术运算符

```ecl
var a = 10;
var b = 3;

var sum = a + b;        // 加法: 13
var diff = a - b;       // 减法: 7
var product = a * b;    // 乘法: 30
var quotient = a / b;   // 除法: 3.333...
```

### 比较运算符

```ecl
var x = 5;
var y = 10;

var eq = x == y;    // 等于: false
var ne = x != y;    // 不等于: true
var lt = x < y;     // 小于: true
var gt = x > y;     // 大于: false
var le = x <= y;    // 小于等于: true
var ge = x >= y;    // 大于等于: false
```

### 逻辑运算符

```ecl
var a = true;
var b = false;

var and = a && b;   // 逻辑与: false
var or = a || b;    // 逻辑或: true
var not = !a;       // 逻辑非: false
```

### 字符串连接

```ecl
var firstName = "John";
var lastName = "Doe";
var fullName = firstName + " " + lastName;  // "John Doe"
```

## 表达式

表达式是由值、变量、运算符和函数调用组成的代码片段：

```ecl
// 简单表达式
var result = 2 + 3 * 4;  // 14

// 复杂表达式
var complex = (10 + 5) * (8 - 3) / 2;  // 37.5

// 函数调用表达式
expr square(l x) {
    return x * x;
}

var value = (5)square + 10;  // 35
```

## 语句

### 表达式语句

```ecl
// 简单的表达式语句
42;
"Hello";
true;
```

### 块语句

块语句由花括号包围的语句序列组成：

```ecl
{
    var x = 10;
    var y = 20;
    var sum = x + y;
    print(sum);
}
```

## 注释

ECL 支持单行注释和多行注释：

```ecl
// 这是单行注释

/*
这是多行注释
可以跨越多行
*/

var x = 10;  // 行尾注释
```

## 空白字符

ECL 忽略多余的空白字符（空格、制表符、换行符）：

```ecl
var x=10;  // 有效
var y = 20 ;  // 有效
var z
=
30
;  // 有效但不推荐
```

## 语句分隔符

语句可以用分号分隔，但不是必需的：

```ecl
// 使用分号（推荐）
var a = 10;
var b = 20;
print(a + b);

// 不使用分号（有效）
var a = 10
var b = 20
print(a + b)
```

## 范围表达式

ECL 支持范围表达式，用于循环和数组索引：

```ecl
// 基本范围
1..5  // 表示 1, 2, 3, 4

// 表达式范围
var start = 1;
var end = 10;
start..end  // 表示 1, 2, 3, ..., 9

// 复杂表达式范围
(5-2)..(3*4)  // 表示 3, 4, 5, 6, 7, 8, 9, 10, 11
```

## 优先级和结合性

运算符具有不同的优先级和结合性：

| 优先级 | 运算符 | 描述 | 结合性 |
|--------|--------|------|--------|
| 1 | `()` | 分组 | 左到右 |
| 2 | `!` | 逻辑非 | 右到左 |
| 3 | `*` `/` | 乘法、除法 | 左到右 |
| 4 | `+` `-` | 加法、减法 | 左到右 |
| 5 | `<` `<=` `>` `>=` | 比较 | 左到右 |
| 6 | `==` `!=` | 相等性 | 左到右 |
| 7 | `&&` | 逻辑与 | 左到右 |
| 8 | `||` | 逻辑或 | 左到右 |

```ecl
// 优先级示例
var result = 2 + 3 * 4;    // 14，不是 20
var result2 = (2 + 3) * 4; // 20，使用括号改变优先级
```

## 错误处理

ECL 提供详细的语法错误信息：

```ecl
// 错误示例
var x = ;  // Syntax error: unexpected semicolon

if (x > 5  // Syntax error: unexpected end of file

// 错误信息会指出具体位置并提供修复建议
```

## 最佳实践

1. **使用分号**：虽然可选，但建议始终使用分号结束语句
2. **合理使用空格**：在运算符周围添加空格以提高可读性
3. **使用括号**：在复杂表达式中使用括号明确优先级
4. **添加注释**：为复杂逻辑添加注释说明
5. **一致的缩进**：使用一致的缩进风格

```ecl
// 推荐写法
var result = (a + b) * (c - d);  // 清晰的优先级
if (age >= 18) {                 // 一致的缩进
    print("Adult");
} else {
    print("Minor");
}

// 不推荐写法
var result=a+b*c-d;  // 难以理解的优先级
if(age>=18){print("Adult")}else{print("Minor")}  // 缺乏缩进
```