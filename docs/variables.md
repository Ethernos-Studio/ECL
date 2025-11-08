# 变量与常量

## 变量声明

ECL 提供了灵活的变量声明机制。

### 基本变量声明

使用 `var` 关键字声明变量，变量值可以修改：

```ecl
// 声明并初始化变量
var x = 10;
var name = "Alice";
var isActive = true;

// 声明后赋值（注意：ECL 中所有变量声明必须包含初始值）
var count = 0;
var message = "Hello";
```

### 类型声明

可以显式指定变量类型：

```ecl
// 显式类型声明
var <int>age = 25;
var <str>username = "user123";
var <bool>isReady = false;
var <float>temperature = 98.6;
var <double>precision = 3.141592653589793;
```

### 数组声明

```ecl
// 声明数组：var <type>name[size] = {initial_value}
var <int>numbers[5] = {0};        // 长度为5的整型数组，全部初始化为0
var <int>scores[3] = {85, 92, 78}; // 长度为3的整型数组，指定初始值
var <str>names[2] = {"Alice", "Bob"}; // 字符串数组
```

### 列表声明

```ecl
// 声明空列表
var items = [];

// 声明带初始值的列表
var mixed = {1, "hello", true, 3.14};
```

## 变量命名规则

变量名必须遵循以下规则：

1. 以字母或下划线开头
2. 后跟字母、数字或下划线
3. 区分大小写
4. 不能使用关键字

```ecl
// 有效的变量名
var name = "Alice";
var _private = 100;
var variable1 = 200;
var firstName = "John";
var MAX_SIZE = 1000;

// 无效的变量名（这些会导致语法错误）
// var 1variable = 10;     // 不能以数字开头
// var my-variable = 20;   // 不能包含连字符
// var if = 30;            // 不能使用关键字
```

## 变量作用域

ECL 目前使用全局作用域，所有变量在整个程序中都可访问：

```ecl
var globalVar = 10;

{
    var localVar = 20;  // 实际上也是全局可访问的
    globalVar = globalVar + 1;  // 可以修改全局变量
}

// localVar 和 globalVar 都可以在这里访问
print(globalVar);  // 11
```

但在函数内部，变量具有局部作用域：

```ecl
func example() {
    var localVar = 100;  // 局部变量
    var <int>count = 5;  // 局部类型声明变量
    return localVar;
}

var result = example();  // result = 100
// localVar 不可在此处访问（实际上是可以的，因为ECL的当前实现）
```

## 变量赋值

### 基本赋值

```ecl
var x = 10;
x = 20;  // 修改变量值

var name = "Alice";
name = "Bob";  // 修改字符串变量
```

### 类型安全赋值

对于有类型声明的变量，赋值时会进行类型检查：

```ecl
var <int>age = 25;
age = 30;        // 合法：同为整数类型
// age = "thirty";  // 错误：类型不匹配

var <str>message = "Hello";
message = "World";  // 合法：同为字符串类型
```

### 数组元素赋值

```ecl
var <int>arr[3] = {1, 2, 3};
arr[0] = 10;    // 修改第一个元素
arr[1] = arr[0] + 5;  // 基于其他元素的值

var lst = {1, "hello", true};
lst[0] = 100;      // 修改列表元素
lst[2] = false;    // 修改布尔值
```

## 常量

ECL 当前没有专门的常量声明关键字，但约定使用全大写命名来表示常量：

```ecl
// 约定：使用全大写表示常量
var PI = 3.14159;
var MAX_USERS = 1000;
var DEFAULT_NAME = "Anonymous";

// 注意：这些实际上仍然是可变的变量
PI = 3.14;  // 这在语法上是允许的，但违反了约定
```

## 特殊变量

### 返回值变量

函数的返回值存储在特殊变量中：

```ecl
func add(a, b) {
    return a + b;
}

var result = add(5, 3);  // result 接收函数返回值
```

## 变量初始化

所有变量声明都必须包含初始值：

```ecl
// 正确的初始化
var count = 0;
var name = "";
var isActive = false;
var <int>score = 0;

// 错误：缺少初始化值
// var uninitialized;  // 语法错误
```

## 数组和列表的特殊初始化

### 数组初始化

```ecl
// 用单个值初始化整个数组
var <int>zeros[5] = {0};      // [0, 0, 0, 0, 0]
var <int>ones[3] = {1};       // [1, 1, 1]

// 用多个值初始化数组
var <int>numbers[4] = {1, 2, 3, 4};  // [1, 2, 3, 4]

// 混合初始化（部分指定值，其余用默认值填充）
var <int>mixed[5] = {10, 20};  // [10, 20, 0, 0, 0]
```

### 列表初始化

```ecl
// 空列表
var empty = [];

// 带初始值的列表
var numbers = {1, 2, 3, 4, 5};
var mixed = {"hello", 42, true, 3.14};
```

## 变量操作

### 变量交换

ECL 不支持直接的变量交换，需要使用临时变量：

```ecl
var a = 10;
var b = 20;

// 使用临时变量交换
var temp = a;
a = b;
b = temp;

print("a = " + a);  // 20
print("b = " + b);  // 10
```

### 变量解构

ECL 目前不支持变量解构赋值。

## 最佳实践

1. **使用有意义的变量名**：
```ecl
// 好的命名
var studentName = "Alice";
var studentAge = 20;
var isStudentActive = true;

// 不好的命名
var x = "Alice";
var y = 20;
var z = true;
```

2. **使用类型声明**：对于重要变量，建议使用类型声明提高代码可读性：
```ecl
var <int>studentCount = 25;
var <str>className = "Mathematics";
```

3. **遵循命名约定**：
   - 使用驼峰命名法：`studentName`, `isActive`
   - 常量使用全大写：`MAX_SIZE`, `DEFAULT_VALUE`

4. **初始化变量**：始终在声明时初始化变量：
```ecl
// 好的做法
var counter = 0;
var message = "";
var isValid = false;

// 不推荐
// 避免后续单独赋值
```

5. **注意类型安全**：对于需要类型安全的场景，使用显式类型声明：
```ecl
var <int>age = 25;      // 确保 age 始终是整数
var <str>name = "Alice"; // 确保 name 始终是字符串
```

## 常见错误和解决方案

### 未初始化变量

```ecl
// 错误示例
// var x;  // 语法错误：变量声明需要初始化值

// 正确做法
var x = 0;  // 或适当的初始值
```

### 类型不匹配

```ecl
// 错误示例
var <int>count = 10;
// count = "ten";  // 运行时类型错误

// 正确做法
var <int>count = 10;
count = 20;  // 同类型赋值
```

### 数组越界访问

```ecl
// 错误示例
var <int>arr[3] = {1, 2, 3};
// var value = arr[5];  // 索引越界错误

// 正确做法
var <int>arr[3] = {1, 2, 3};
var value = arr[0];  // 合法访问
```

## 总结

ECL 的变量系统提供了灵活的声明和使用方式，支持：
- 基本变量声明和赋值
- 显式类型声明
- 数组和列表数据结构
- 类型安全检查
- 明确的初始化要求

通过遵循最佳实践，您可以编写出清晰、安全和易于维护的 ECL 代码。