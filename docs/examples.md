# 示例与最佳实践

本章节提供了一系列 ECL 代码示例和编程最佳实践，帮助您更好地理解和使用 ECL 语言。

## 基础示例

### 1. Hello World

```ecl
// 最简单的 ECL 程序
print("Hello, World!");
```

### 2. 变量和基本运算

```ecl
// 变量声明和使用
var name = "Alice";
var age = 25;
var height = 5.6;

print("Name: ");
print(name);
println("");
print("Age: ");
print(age);
println("");
print("Height: ");
print(height);
println(" feet");
```

### 3. 用户输入和输出

```ecl
// 交互式程序
var userName = "";
input("What's your name? ", userName);

var userAge = "";
input("How old are you? ", userAge);

println("Nice to meet you, " + userName + "!");
print("You are ");
print(userAge);
print(" years old.");
println("");
```

## 控制流示例

### 1. 条件语句

```ecl
// 成绩等级判断
func getGrade(score) {
    if (score >= 90) {
        return "A";
    } else if (score >= 80) {
        return "B";
    } else if (score >= 70) {
        return "C";
    } else if (score >= 60) {
        return "D";
    } else {
        return "F";
    }
}

var studentScore = 85;
var grade = getGrade(studentScore);
print("Score: ");
print(studentScore);
print(", Grade: ");
print(grade);
println("");
```

### 2. 循环语句

```ecl
// 使用 for 循环计算阶乘
func factorial(n) {
    var result = 1;
    for i in 1..(n+1) {
        result = result * i;
    }
    return result;
}

// 计算并显示 1! 到 5! 的值
for i in 1..6 {
    var fact = factorial(i);
    print(i);
    print("! = ");
    print(fact);
    println("");
}
```

### 3. while 循环

```ecl
// 使用 while 循环实现猜数字游戏
func guessNumberGame() {
    var target = 42;  // 假设目标数字是 42
    var guess = 0;
    var attempts = 0;
    
    println("Guess the number (1-100):");
    
    while (guess != target) {
        var inputStr = "";
        input("Your guess: ", inputStr);
        guess = <int>inputStr;
        attempts = attempts + 1;
        
        if (guess < target) {
            println("Too low! Try again.");
        } else if (guess > target) {
            println("Too high! Try again.");
        } else {
            print("Correct! You guessed it in ");
            print(attempts);
            print(" attempts.");
            println("");
        }
    }
}

// guessNumberGame();
```

## 函数示例

### 1. 普通函数

```ecl
// 数学计算函数
func calculateRectangleArea(length, width) {
    return length * width;
}

func calculateCircleArea(radius) {
    return 3.14159 * radius * radius;
}

// 使用函数
var rectArea = calculateRectangleArea(10, 5);
var circleArea = calculateCircleArea(3);

print("Rectangle area: ");
print(rectArea);
println("");
print("Circle area: ");
print(circleArea);
println("");
```

### 2. 表达式函数

```ecl
// 定义表达式函数
expr power(l base, r exponent) {
    var result = 1;
    for i in 1..(exponent+1) {
        result = result * base;
    }
    return result;
}

expr max(l a, r b) {
    if (a > b) {
        return a;
    }
    return b;
}

expr min(l a, r b) {
    if (a < b) {
        return a;
    }
    return b;
}

// 使用表达式函数
var result1 = 2 power 8;  // 256
var maxValue = 10 max 15; // 15
var minValue = 10 min 15; // 10

print("2^8 = ");
print(result1);
println("");
print("Max of 10 and 15: ");
print(maxValue);
println("");
print("Min of 10 and 15: ");
print(minValue);
println("");
```

## 数据结构示例

### 1. 数组操作

```ecl
// 数组示例：学生成绩管理
func demonstrateArray() {
    // 声明并初始化学生成绩数组
    var <int>grades[5] = {85, 92, 78, 96, 88};
    
    // 显示所有成绩
    println("Student Grades:");
    for i in 0..5 {
        print("Student ");
        print(i+1);
        print(": ");
        print(grades[i]);
        println("");
    }
    
    // 计算平均分
    var sum = 0;
    for i in 0..5 {
        sum = sum + grades[i];
    }
    var average = sum / 5;
    
    print("Average grade: ");
    print(average);
    println("");
    
    // 找出最高分和最低分
    var highest = grades[0];
    var lowest = grades[0];
    
    for i in 1..5 {
        if (grades[i] > highest) {
            highest = grades[i];
        }
        if (grades[i] < lowest) {
            lowest = grades[i];
        }
    }
    
    print("Highest grade: ");
    print(highest);
    println("");
    print("Lowest grade: ");
    print(lowest);
    println("");
}

demonstrateArray();
```

### 2. 列表操作

```ecl
// 列表示例：混合数据存储
func demonstrateList() {
    // 创建包含不同数据类型的列表
    var person = {"Alice", 25, true, "Engineer", 5.2};
    
    // 访问列表元素
    var name = person[0];
    var age = person[1];
    var isEmployed = person[2];
    var job = person[3];
    var height = person[4];
    
    // 显示个人信息
    println("Person Information:");
    print("Name: ");
    print(name);
    println("");
    print("Age: ");
    print(age);
    println("");
    print("Employed: ");
    print(isEmployed);
    println("");
    print("Job: ");
    print(job);
    println("");
    print("Height: ");
    print(height);
    print(" feet");
    println("");
    
    // 修改列表元素
    person[1] = 26;  // 更新年龄
    person[3] = "Senior Engineer";  // 更新职位
    
    println("Updated Information:");
    print("Age: ");
    print(person[1]);
    println("");
    print("Job: ");
    print(person[3]);
    println("");
}

demonstrateList();
```

## 类型转换示例

```ecl
// 类型转换示例
func demonstrateTypeConversion() {
    // 字符串到数字
    var strNum = "123";
    var intNum = <int>strNum;
    var floatNum = <float>strNum;
    
    print("String '");
    print(strNum);
    print("' to int: ");
    print(intNum);
    println("");
    print("String '");
    print(strNum);
    print("' to float: ");
    print(floatNum);
    println("");
    
    // 数字到字符串
    var number = 456;
    var numStr = <str>number;
    
    print("Number ");
    print(number);
    print(" to string: '");
    print(numStr);
    print("'");
    println("");
    
    // 布尔转换
    var truth = true;
    var falseVal = false;
    
    var intFromTrue = <int>truth;    // 1
    var intFromFalse = <int>falseVal; // 0
    var strFromTrue = <str>truth;    // "true"
    var strFromFalse = <str>falseVal; // "false"
    
    print("true to int: ");
    print(intFromTrue);
    println("");
    print("false to int: ");
    print(intFromFalse);
    println("");
    print("true to string: '");
    print(strFromTrue);
    print("'");
    println("");
    print("false to string: '");
    print(strFromFalse);
    print("'");
    println("");
}

demonstrateTypeConversion();
```

## 实用工具函数

### 1. 数学工具

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

func clamp(value, minVal, maxVal) {
    if (value < minVal) {
        return minVal;
    }
    if (value > maxVal) {
        return maxVal;
    }
    return value;
}

// 使用示例
var testValue = -15;
var absolute = abs(testValue);
print("Absolute value of ");
print(testValue);
print(" is ");
print(absolute);
println("");

var larger = max(10, 20);
var smaller = min(10, 20);
print("Max of 10 and 20: ");
print(larger);
println("");
print("Min of 10 and 20: ");
print(smaller);
println("");

var clamped = clamp(15, 10, 20);  // 15
var clampedLow = clamp(5, 10, 20);   // 10
var clampedHigh = clamp(25, 10, 20); // 20
print("Clamped values: ");
print(clamped);
print(", ");
print(clampedLow);
print(", ");
print(clampedHigh);
println("");
```

### 3. 转义序列示例

```ecl
// 转义序列使用示例
func demonstrateEscapeSequences() {
    println("=== Escape Sequences Demo ===");
    
    // 换行符
    println("Line 1\nLine 2\nLine 3");
    
    // 制表符
    println("Name\tAge\tCity");
    println("Alice\t25\tNew York");
    println("Bob\t30\tLondon");
    
    // 引号
    println("She said \"Hello World!\"");
    println('It\'s a beautiful day');
    
    // 反斜杠
    println("Windows path: C:\\Users\\Documents");
    println("Unix path: /home/user/documents");
    
    // Unicode转义
    println("Emoji: \u{1F600} \u{1F601} \u{1F602}");
    println("Chinese: \u{4E2D}\u{6587}\u{6D4B}\u{8BD5}");
    println("Math: \u{03C0} = 3.14159");
    
    // 混合使用
    println("Mixed: Hello\u0020World!\nTab:\u0009Here");
}

demonstrateEscapeSequences();
```

### 4. 数组和列表输出示例

```ecl
// 数组和列表的正确输出方式
func demonstrateArrayOutput() {
    println("=== Array and List Output ===");
    
    // 数组输出（需要类型转换）
    var <int>scores[3] = {85, 92, 78};
    println("First score: " + <str>scores[0]);
    println("All scores: " + <str>scores[0] + ", " + <str>scores[1] + ", " + <str>scores[2]);
    
    // 使用循环输出数组
    println("Using loop:");
    for i in 0..3 {
        print("Score ");
        print(i);
        print(": ");
        println(<str>scores[i]);
    }
    
    // 列表输出（需要类型转换）
    var mixed = {1, "hello", true, 3.14};
    println("List elements:");
    println("First: " + <str>mixed[0]);
    println("String: " + mixed[3]);  // 字符串不需要转换
    println("Number: " + <str>mixed[0]);
    println("Boolean: " + <str>mixed[2]);
}

demonstrateArrayOutput();
```

### 2. 字符串工具

```ecl
// 字符串工具函数
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

func length(str) {
    // 简化实现，实际需要更复杂的逻辑
    // 这里假设我们有一个获取字符串长度的函数
    return getStringLength(str);  // 假设函数存在
}

// 使用示例
var greeting = concat("Hello", " World!");
print(greeting);
println("");

var repeated = repeat("*", 5);
print("Repeated: ");
print(repeated);
println("");

var strLen = length("ECL Programming");
print("Length of 'ECL Programming': ");
print(strLen);
println("");
```

## 综合示例

### 1. 简单计算器

```ecl
// 简单计算器应用
func simpleCalculator() {
    println("=== Simple Calculator ===");
    println("Supported operations: +, -, *, /");
    println("Enter 'quit' to exit");
    println("");
    
    var running = true;
    
    while (running) {
        var num1Str = "";
        var operator = "";
        var num2Str = "";
        
        input("Enter first number: ", num1Str);
        if (num1Str == "quit") {
            running = false;
            break;
        }
        
        input("Enter operator (+, -, *, /): ", operator);
        if (operator == "quit") {
            running = false;
            break;
        }
        
        input("Enter second number: ", num2Str);
        if (num2Str == "quit") {
            running = false;
            break;
        }
        
        var num1 = <float>num1Str;
        var num2 = <float>num2Str;
        var result = 0.0;
        
        if (operator == "+") {
            result = num1 + num2;
        } else if (operator == "-") {
            result = num1 - num2;
        } else if (operator == "*") {
            result = num1 * num2;
        } else if (operator == "/") {
            if (num2 != 0) {
                result = num1 / num2;
            } else {
                println("Error: Division by zero!");
                continue;
            }
        } else {
            println("Error: Invalid operator!");
            continue;
        }
        
        print("Result: ");
        print(result);
        println("");
        println("");
    }
    
    println("Goodbye!");
}

// simpleCalculator();
```

### 2. 学生成绩管理系统

```ecl
// 学生成绩管理系统
func studentGradeSystem() {
    println("=== Student Grade Management System ===");
    
    // 使用列表存储学生信息
    var students = {};
    var studentCount = 0;
    
    var running = true;
    
    while (running) {
        println("");
        println("1. Add student");
        println("2. View all students");
        println("3. Calculate average");
        println("4. Exit");
        print("Choose an option: ");
        
        var choice = "";
        input("", choice);
        
        if (choice == "1") {
            // 添加学生
            var name = "";
            var gradeStr = "";
            
            input("Enter student name: ", name);
            input("Enter student grade: ", gradeStr);
            
            var grade = <int>gradeStr;
            
            // 创建学生记录并添加到列表
            var student = {name, grade};
            students[studentCount] = student;
            studentCount = studentCount + 1;
            
            println("Student added successfully!");
            
        } else if (choice == "2") {
            // 查看所有学生
            if (studentCount == 0) {
                println("No students found.");
            } else {
                println("Student List:");
                for i in 0..studentCount {
                    var student = students[i];
                    var name = student[0];
                    var grade = student[1];
                    print(i+1);
                    print(". ");
                    print(name);
                    print(" - Grade: ");
                    print(grade);
                    println("");
                }
            }
            
        } else if (choice == "3") {
            // 计算平均分
            if (studentCount == 0) {
                println("No students found.");
            } else {
                var sum = 0;
                for i in 0..studentCount {
                    var student = students[i];
                    var grade = student[1];
                    sum = sum + grade;
                }
                var average = sum / studentCount;
                print("Average grade: ");
                print(average);
                println("");
            }
            
        } else if (choice == "4") {
            // 退出
            running = false;
            println("Goodbye!");
            
        } else {
            println("Invalid option. Please try again.");
        }
    }
}

// studentGradeSystem();
```

## 最佳实践

### 1. 命名约定

```ecl
// 好的命名
var studentName = "Alice";
var calculateArea = func(length, width) {
    return length * width;
};
var MAX_RETRY_COUNT = 3;

// 表达式函数使用描述性名称
expr calculatePower(l base, r exponent) {
    var result = 1;
    for i in 1..(exponent+1) {
        result = result * base;
    }
    return result;
}

// 不好的命名
var x = "Alice";
var calc = func(a, b) {
    return a * b;
};
var m = 3;
```

### 2. 代码组织

```ecl
// 将相关的函数和变量组织在一起
// 数学工具模块
func add(a, b) {
    return a + b;
}

func subtract(a, b) {
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

// 字符串工具模块
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

### 3. 错误处理

```ecl
// 良好的错误处理
func safeDivide(dividend, divisor) {
    if (divisor == 0) {
        print("Error: Division by zero is not allowed.");
        println("");
        return 0;
    }
    return dividend / divisor;
}

func validateAge(ageStr) {
    var age = <int>ageStr;
    if (age < 0 || age > 150) {
        print("Warning: Age ");
        print(age);
        print(" seems invalid. Using default value 0.");
        println("");
        return 0;
    }
    return age;
}
```

### 4. 注释和文档

```ecl
// 计算两个数的最大公约数
// 使用欧几里得算法
// 参数:
//   a - 第一个整数
//   b - 第二个整数
// 返回:
//   两个数的最大公约数
func gcd(a, b) {
    // 确保 a >= b
    if (a < b) {
        var temp = a;
        a = b;
        b = temp;
    }
    
    // 欧几里得算法
    while (b != 0) {
        var remainder = a % b;
        a = b;
        b = remainder;
    }
    
    return a;
}

// 表达式函数也需要注释
// 计算数字的平方
// 参数:
//   r x - 要计算平方的数字
// 返回:
//   x 的平方值
expr square(r x) {
    return x * x;
}
```

### 5. 性能优化

```ecl
// 避免在循环中重复计算
func inefficientSum(n) {
    var sum = 0;
    for i in 1..(n+1) {
        sum = sum + (i * i);  // 每次都计算 i*i
    }
    return sum;
}

// 优化版本
func efficientSum(n) {
    var sum = 0;
    for i in 1..(n+1) {
        var square = i * i;  // 只计算一次
        sum = sum + square;
    }
    return sum;
}

// 预计算常量
var PI = 3.141592653589793;
var E = 2.718281828459045;

func circleArea(radius) {
    return PI * radius * radius;  // 使用预计算的常量
}
```

## 代码复用示例

### 1. 工具库概念

```ecl
// 创建一个简单的工具库概念
// 数学工具
func math_abs(value) {
    if (value < 0) {
        return -value;
    }
    return value;
}

func math_max(a, b) {
    if (a > b) {
        return a;
    }
    return b;
}

func math_min(a, b) {
    if (a < b) {
        return a;
    }
    return b;
}

// 字符串工具
func string_concat(a, b) {
    return a + b;
}

func string_repeat(str, times) {
    var result = "";
    for i in 1..(times+1) {
        result = result + str;
    }
    return result;
}

// 使用工具函数
var distance = math_abs(10 - 15);  // 5
var largest = math_max(20, 30);    // 30
var greeting = string_concat("Hello", " World!");  // "Hello World!"
```

### 2. 配置管理

```ecl
// 简单的配置管理
var config = {
    "app_name": "ECL Application",
    "version": "1.0.0",
    "debug": "true",
    "max_users": "100"
};

func getConfig(key) {
    // 简化实现，实际需要查找机制
    return config[key];
}

func setConfig(key, value) {
    config[key] = value;
}

// 使用配置
var appName = getConfig("app_name");
var debugMode = <bool>getConfig("debug");
var maxUsers = <int>getConfig("max_users");

print("Application: ");
print(appName);
println("");
print("Debug mode: ");
print(debugMode);
println("");
print("Max users: ");
print(maxUsers);
println("");
```

## 测试示例

### 1. 简单测试

```ecl
// 简单的测试函数
func test_add() {
    var result = add(2, 3);
    if (result == 5) {
        println("✓ add(2, 3) = 5 - PASSED");
    } else {
        print("✗ add(2, 3) = ");
        print(result);
        print(" (expected 5) - FAILED");
        println("");
    }
}

func test_multiply() {
    var result = (4)multiply(5);
    if (result == 20) {
        println("✓ 4 multiply 5 = 20 - PASSED");
    } else {
        print("✗ 4 multiply 5 = ");
        print(result);
        print(" (expected 20) - FAILED");
        println("");
    }
}

// 运行测试
println("Running tests...");
test_add();
test_multiply();
println("Tests completed.");
```

## 总结

通过这些示例，您可以：

1. **掌握基础语法**：变量、控制流、函数等
2. **理解高级特性**：表达式函数、数据结构等
3. **学习最佳实践**：命名、组织、错误处理等
4. **应用实际场景**：计算器、管理系统等

记住，最好的学习方式是实践。尝试修改这些示例，创建自己的程序，并在 REPL 中测试不同的想法。随着经验的积累，您将能够编写出更加复杂和高效的 ECL 程序。