# 表达式函数

表达式函数是 ECL 的独特特性，允许定义可以使用中缀语法调用的函数，使代码更接近数学表达式的自然写法。

## 表达式函数定义

使用 `expr` 关键字定义表达式函数：

```ecl
// 基本表达式函数定义
expr square(l x) {
    return x * x;
}

// 多参数表达式函数
expr add(l a, r b) {
    return a + b;
}

// 三个参数的表达式函数
expr calculate(l1 a, l2 b, r c) {
    return a * b + c;
}
```

## 参数类型

表达式函数的参数具有特定的类型标识符：

### 左参数 (l)

```ecl
expr negate(l x) {
    return -x;
}

expr increment(l x) {
    return x + 1;
}

// 调用
var result = 5 negate;    // -5
var value = 10 increment; // 11
```

### 右参数 (r)

```ecl
expr square(r x) {
    return x * x;
}

expr double(r x) {
    return x * 2;
}

// 调用
var result = 5 square;  // 25
var value = 10 double;  // 20
```

### 左右参数 (l, r)

```ecl
expr add(l a, r b) {
    return a + b;
}

expr multiply(l a, r b) {
    return a * b;
}

expr divide(l a, r b) {
    if (b == 0) {
        return 0;
    }
    return a / b;
}

// 调用
var sum = 2 add 3;       // 5
var product = 4 multiply 5; // 20
var quotient = 10 divide 2; // 5
```

### 多左参数

```ecl
expr complex(l1 a, l2 b, r c) {
    return a * b + c;
}

// 调用
var result = 2 complex 3 10;  // 2 * 3 + 10 = 16
```

### 可变参数 (argv)

```ecl
expr sum(l first, r argv) {
    var total = first;
    // 遍历 argv 中的参数
    var length = argv.size();  // 假设支持 size() 方法
    for i in 0..length {
        total = total + argv[i];
    }
    return total;
}

// 调用
var result = 1 sum 2 3 4;  // 1 + 2 + 3 + 4 = 10
```

## 调用语法

表达式函数支持多种调用方式：

### 中缀调用（推荐）

```ecl
expr add(l a, r b) {
    return a + b;
}

expr multiply(l a, r b) {
    return a * b;
}

// 中缀调用
var sum = 5 add 3;        // 8
var product = 4 multiply 6; // 24
var complex = 2 add 3 multiply 4;  // 2 + (3 * 4) = 14
```

### 括号调用

```ecl
// 括号调用（更明确）
var sum = (5)add(3);        // 8
var product = (4)multiply(6); // 24

// 混合调用
var result = (2 add 3) multiply (4 square);  // (2+3) * (4²) = 5 * 16 = 80
```

## 实际应用示例

### 数学运算

```ecl
// 幂运算
expr power(l base, r exponent) {
    var result = 1;
    for i in 1..(exponent+1) {
        result = result * base;
    }
    return result;
}

var value = 2 power 8;  // 256

// 模运算
expr mod(l dividend, r divisor) {
    return dividend - (dividend / divisor) * divisor;
}

var remainder = 17 mod 5;  // 2
```

### 字符串操作

```ecl
// 字符串重复
expr repeat(l str, r times) {
    var result = "";
    for i in 1..(times+1) {
        result = result + str;
    }
    return result;
}

var text = "Hello" repeat 3;  // "HelloHelloHello"

// 字符串连接
expr append(l str1, r str2) {
    return str1 + str2;
}

var greeting = "Hello, " append "World!";  // "Hello, World!"
```

### 比较操作

```ecl
// 最大值
expr max(l a, r b) {
    if (a > b) {
        return a;
    }
    return b;
}

var maximum = 10 max 15;  // 15

// 最小值
expr min(l a, r b) {
    if (a < b) {
        return a;
    }
    return b;
}

var minimum = 10 min 15;  // 10
```

### 逻辑操作

```ecl
// 逻辑与（数值表示）
expr and(l a, r b) {
    if (a != 0 && b != 0) {
        return 1;
    }
    return 0;
}

var result = 1 and 1;  // 1 (true)

// 逻辑或（数值表示）
expr or(l a, r b) {
    if (a != 0 || b != 0) {
        return 1;
    }
    return 0;
}

var result = 0 or 1;  // 1 (true)
```

## 优先级和结合性

表达式函数调用遵循特定的优先级和结合性规则：

```ecl
expr add(l a, r b) {
    return a + b;
}

expr multiply(l a, r b) {
    return a * b;
}

// 优先级：multiply 比 add 优先级高
var result = 2 add 3 multiply 4;  // 2 + (3 * 4) = 14

// 使用括号改变优先级
var result2 = (2 add 3) multiply 4;  // (2 + 3) * 4 = 20

// 左结合性
var result3 = 10 add 5 add 3;  // (10 add 5) add 3 = 18
```

## 嵌套调用

表达式函数支持嵌套调用：

```ecl
expr square(r x) {
    return x * x;
}

expr add(l a, r b) {
    return a + b;
}

expr multiply(l a, r b) {
    return a * b;
}

// 嵌套调用
var result = (2 add 3) square;        // (2 + 3)² = 25
var complex = (2 square) add (3 square); // 2² + 3² = 4 + 9 = 13
var nested = ((2 add 3) multiply 4) square; // ((2 + 3) * 4)² = (20)² = 400
```

## 错误处理

表达式函数中的错误处理：

```ecl
expr safeDivide(l dividend, r divisor) {
    if (divisor == 0) {
        print("Error: Division by zero");
        println("");
        return 0;
    }
    return dividend / divisor;
}

var result = 10 safeDivide 0;  // 错误处理，返回 0
```

## 最佳实践

### 1. 命名约定

使用描述性的名称：

```ecl
// 好的命名
expr square(r x) {
    return x * x;
}

expr calculateArea(l length, r width) {
    return length * width;
}

// 不好的命名
expr sq(r x) {
    return x * x;
}

expr mult(l a, r b) {
    return a * b;
}
```

### 2. 参数顺序

合理安排参数顺序以提高可读性：

```ecl
// 合理的参数顺序
expr subtract(l minuend, r subtrahend) {
    return minuend - subtrahend;
}

expr divide(l dividend, r divisor) {
    return dividend / divisor;
}
```

### 3. 一致性

在项目中保持一致的表达式函数使用：

```ecl
// 数学运算表达式函数
expr add(l a, r b) {
    return a + b;
}

expr subtract(l a, r b) {
    return a - b;
}

expr multiply(l a, r b) {
    return a * b;
}

expr divide(l a, r b) {
    if (b == 0) {
        return 0;
    }
    return a / b;
}

// 使用
var result = 10 add 5 multiply 2;  // 10 + (5 * 2) = 20
```

### 4. 文档化

为复杂的表达式函数添加注释：

```ecl
// 计算两点之间的距离
// 参数：l x1 - 第一个点的x坐标
// 参数：l y1 - 第一个点的y坐标
// 参数：r x2 - 第二个点的x坐标
// 参数：r y2 - 第二个点的y坐标
expr distance(l1 x1, l2 y1, r1 x2, r2 y2) {
    var dx = x2 - x1;
    var dy = y2 - y1;
    return (dx * dx + dy * dy) square;  // 假设 square 函数已定义
}
```

## 性能考虑

### 1. 避免复杂计算

```ecl
// 避免在表达式函数中进行复杂计算
expr fastOperation(l a, r b) {
    return a + b;  // 简单操作
}

// 而不是
expr slowOperation(l a, r b) {
    // 复杂的计算
    var result = 0;
    for i in 1..1000 {
        result = result + (a * b) / i;
    }
    return result;
}
```

### 2. 缓存结果

```ecl
// 对于重复计算，考虑缓存
var cachedResults = [];

expr expensiveCalculation(r x) {
    // 检查缓存
    if (cachedResults.size() > x && cachedResults[x] != 0) {
        return cachedResults[x];
    }
    
    // 执行计算
    var result = x * x * x;  // 假设是复杂计算
    
    // 缓存结果
    if (cachedResults.size() <= x) {
        // 扩展数组
        for i in cachedResults.size()..(x+1) {
            cachedResults.push(0);
        }
    }
    cachedResults[x] = result;
    
    return result;
}
```

## 与其他语言特性的结合

### 与数组结合

```ecl
expr atIndex(l array, r index) {
    return array[index];
}

var <int>numbers[5] = {10, 20, 30, 40, 50};
var value = numbers atIndex 2;  // 30
```

### 与控制流结合

```ecl
expr isEven(r number) {
    if (number % 2 == 0) {
        return 1;  // true
    }
    return 0;  // false
}

var result = 4 isEven;  // 1 (true)
```

## 常见错误和解决方案

### 1. 参数类型错误

```ecl
expr add(l a, r b) {
    return a + b;
}

// 错误：参数类型不匹配
// var result = "hello" add 5;  // 字符串和数字不能直接相加

// 解决方案：确保参数类型兼容
var result = "hello" add "5";  // 如果 add 支持字符串连接
```

### 2. 优先级混淆

```ecl
expr add(l a, r b) {
    return a + b;
}

expr multiply(l a, r b) {
    return a * b;
}

// 可能的混淆
var result = 2 add 3 multiply 4;  // 实际是 2 + (3 * 4) = 14

// 明确的写法
var result2 = 2 add (3 multiply 4);  // 14
var result3 = (2 add 3) multiply 4;  // 20
```

## 总结

表达式函数是 ECL 的核心特性之一，提供了：

- **自然的语法**：中缀调用使代码更接近数学表达式
- **灵活性**：多种参数类型支持不同的使用场景
- **可读性**：使复杂表达式更容易理解
- **可扩展性**：可以定义自定义的操作符

通过合理使用表达式函数，您可以编写出更加直观和优雅的 ECL 代码。记住要遵循最佳实践，保持命名一致性，并注意表达式的优先级和结合性。