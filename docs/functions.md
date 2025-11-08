# 函数

函数是 ECL 中组织代码的基本单元，允许将代码逻辑封装成可重用的模块。

## 函数定义

### 普通函数

使用 `func` 关键字定义普通函数：

```ecl
// 基本函数定义
func greet() {
    print("Hello, World!");
}

// 带参数的函数
func sayHello(name) {
    print("Hello, ");
    print(name);
    println("!");
}

// 带返回值的函数
func add(a, b) {
    return a + b;
}

// 复杂函数示例
func calculateArea(length, width) {
    var area = length * width;
    return area;
}
```

### 表达式函数

使用 `expr` 关键字定义表达式函数，支持中缀调用语法：

```ecl
// 表达式函数定义
expr square(l x) {
    return x * x;
}

// 多参数表达式函数
expr multiply(l a, r b) {
    return a * b;
}

// 三个参数的表达式函数
expr addThree(l1 a, l2 b, r c) {
    return a + b + c;
}
```

## 函数参数

### 普通函数参数

普通函数参数通过逗号分隔：

```ecl
// 无参数
func noParams() {
    print("No parameters");
}

// 单参数
func oneParam(x) {
    print(x);
}

// 多参数
func multiParams(a, b, c) {
    print(a + b + c);
}
```

### 表达式函数参数

表达式函数参数具有类型标识符：

```ecl
// 左参数
expr negate(l x) {
    return -x;
}

// 右参数
expr square(r x) {
    return x * x;
}

// 左右参数
expr add(l a, r b) {
    return a + b;
}

// 多个左参数和一个右参数
expr complex(l1 a, l2 b, r c) {
    return a * b + c;
}

// 可变参数（argv）
expr sum(l first, r argv) {
    var total = first;
    // argv 是一个包含额外参数的列表
    for i in 0..argv.size() {
        total = total + argv[i];
    }
    return total;
}
```

## 函数调用

### 普通函数调用

```ecl
// 无参数调用
greet();

// 带参数调用
sayHello("Alice");

// 带返回值调用
var result = add(5, 3);
print("Result: " + result);

// 链式调用
var area = calculateArea(10, 5);
var doubled = (area)multiply(2);
```

### 表达式函数调用

表达式函数支持多种调用方式：

```ecl
// 中缀调用（推荐）
var squared = 5 square;
var product = 3 multiply 4;
var sum = 2 add 3;

// 括号调用
var squared2 = (5)square;
var product2 = (3)multiply(4);
var sum2 = (2)add(3);

// 复杂表达式调用
var result = (2 add 3) multiply (4 square);
```

## 返回值

### 返回语句

使用 `return` 关键字返回值：

```ecl
// 返回简单值
func getValue() {
    return 42;
}

// 返回表达式结果
func calculate(x, y) {
    return x * y + 10;
}

// 条件返回
func max(a, b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

// 提前返回
func process(value) {
    if (value < 0) {
        return 0;  // 提前返回
    }
    
    return value * 2;
}
```

### 无返回值函数

如果函数没有明确的返回语句，默认返回 0：

```ecl
func printMessage(msg) {
    print("Message: ");
    print(msg);
    println("");
    // 隐式返回 0
}
```

## 作用域和生命周期

### 局部变量

函数内部声明的变量具有局部作用域：

```ecl
func example() {
    var localVar = 100;  // 局部变量
    var <int>count = 5;  // 局部类型声明变量
    
    // 这些变量只在函数内部可见
    return localVar;
}

var result = example();  // result = 100
// localVar 在此处不可访问
```

### 参数传递

ECL 中的参数传递是按值传递：

```ecl
func modifyValue(x) {
    x = x * 2;  // 只修改局部副本
    return x;
}

var original = 10;
var modified = modifyValue(original);
// original 仍然是 10，modified 是 20
```

## 递归函数

ECL 支持递归函数调用：

```ecl
// 计算阶乘
func factorial(n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

var result = factorial(5);  // 120

// 计算斐波那契数列
func fibonacci(n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

var fib = fibonacci(10);  // 55
```

## 高阶函数

虽然 ECL 不直接支持函数作为参数传递，但可以通过表达式函数实现类似功能：

```ecl
// 定义操作函数
expr addOp(l a, r b) {
    return a + b;
}

expr multiplyOp(l a, r b) {
    return a * b;
}

// 使用表达式函数
var sum = 5 addOp 3;       // 8
var product = 5 multiplyOp 3;  // 15
```

## 内置函数

ECL 提供了一些内置函数：

```ecl
// 输入函数
input("Enter your name: ", name);

// 输出函数
print("Hello");
println("World");

// 类型转换函数（通过语法实现）
var num = 123;
var str = <str>num;
```

## 函数重载

ECL 不支持函数重载，每个函数名必须唯一：

```ecl
// 错误：不能定义同名函数
// func calculate(a, b) { return a + b; }
// func calculate(a, b, c) { return a + b + c; }  // 语法错误

// 正确：使用不同名称
func calculate2(a, b) {
    return a + b;
}

func calculate3(a, b, c) {
    return a + b + c;
}
```

## 最佳实践

### 1. 函数命名

使用描述性的函数名：

```ecl
// 好的命名
func calculateCircleArea(radius) {
    return 3.14159 * radius * radius;
}

func isValidEmail(email) {
    // 验证逻辑
    return true;
}

// 不好的命名
func calc(r) {
    return 3.14159 * r * r;
}

func check(e) {
    return true;
}
```

### 2. 函数长度

保持函数简短，专注于单一职责：

```ecl
// 好的做法：短小精悍的函数
func square(x) {
    return x * x;
}

func isEven(number) {
    return number % 2 == 0;
}

// 组合使用
func processNumber(n) {
    if (isEven(n)) {
        return square(n);
    }
    return n;
}
```

### 3. 参数数量

尽量减少函数参数数量：

```ecl
// 好的做法：参数较少
func add(a, b) {
    return a + b;
}

// 如果需要多个参数，考虑使用对象或数组
func processPoint(x, y) {
    // 处理点坐标
    return x * y;
}
```

### 4. 表达式函数使用

合理使用表达式函数以提高代码可读性：

```ecl
// 定义有意义的表达式函数
expr power(l base, r exponent) {
    var result = 1;
    for i in 1..(exponent+1) {
        result = result * base;
    }
    return result;
}

// 使用中缀语法使表达式更自然
var result = 2 power 8;  // 256
```

### 5. 错误处理

在函数中适当处理错误情况：

```ecl
func divide(a, b) {
    if (b == 0) {
        print("Error: Division by zero");
        return 0;
    }
    return a / b;
}
```

## 常见模式

### 1. 工具函数

```ecl
// 数学工具函数
func abs(value) {
    if (value < 0) {
        return -value;
    }
    return value;
}

func max(a, b) {
    if (a > b) {
        return a;
    }
    return b;
}

func min(a, b) {
    if (a < b) {
        return a;
    }
    return b;
}
```

### 2. 字符串处理函数

```ecl
func concat(a, b) {
    return a + b;
}

func repeat(str, times) {
    var result = "";
    for i in 1..(times+1) {
        result = result + str;
    }
    return result;
}
```

### 3. 数组处理函数

```ecl
// 计算数组元素之和
func sumArray(arr) {
    var sum = 0;
    // 注意：需要数组长度信息
    return sum;
}

// 查找数组中的最大值
func maxInArray(arr) {
    if (arr.size() == 0) {
        return 0;
    }
    
    var max = arr[0];
    for i in 1..arr.size() {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}
```

## 调试技巧

### 1. 添加调试输出

```ecl
func complexCalculation(a, b, c) {
    print("Input: a=");
    print(a);
    print(", b=");
    print(b);
    print(", c=");
    print(c);
    println("");
    
    var result = a * b + c;
    
    print("Result: ");
    print(result);
    println("");
    
    return result;
}
```

### 2. 分步调试

```ecl
func process(data) {
    // 步骤1：验证输入
    if (data < 0) {
        print("Invalid input: ");
        print(data);
        println("");
        return 0;
    }
    
    // 步骤2：处理数据
    var processed = data * 2;
    
    // 步骤3：返回结果
    return processed;
}
```

## 性能考虑

### 1. 避免深度递归

```ecl
// 可能导致栈溢出的深度递归
func badRecursion(n) {
    if (n <= 0) {
        return 0;
    }
    return n + badRecursion(n - 1);
}

// 使用循环替代
func goodIteration(n) {
    var sum = 0;
    for i in 1..(n+1) {
        sum = sum + i;
    }
    return sum;
}
```

### 2. 缓存计算结果

```ecl
// 对于重复计算，考虑缓存结果
var cachedFactorial = [];

func factorialWithCache(n) {
    if (n <= 1) {
        return 1;
    }
    
    // 检查缓存
    if (cachedFactorial.size() > n && cachedFactorial[n] != 0) {
        return cachedFactorial[n];
    }
    
    var result = n * factorialWithCache(n - 1);
    
    // 缓存结果
    if (cachedFactorial.size() <= n) {
        // 扩展数组
        for i in cachedFactorial.size()..(n+1) {
            cachedFactorial.push(0);
        }
    }
    cachedFactorial[n] = result;
    
    return result;
}
```

## 总结

ECL 的函数系统提供了：
- 普通函数和表达式函数两种定义方式
- 灵活的参数机制
- 中缀调用语法提高表达式可读性
- 递归支持
- 类型安全的返回值处理

通过合理使用函数，您可以创建模块化、可重用和易于维护的 ECL 程序。