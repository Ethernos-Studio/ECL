# 输入输出

ECL 提供了基本的输入输出功能，用于与用户交互和显示程序结果。

## 输出函数

### print 函数

`print` 函数输出内容但不换行：

```ecl
// 输出字符串
print("Hello, World!");

// 输出数字
print(42);

// 输出布尔值
print(true);

// 连续输出
print("The value is: ");
print(123);
print(" and it's ");
print(true);
// 输出: The value is: 123 and it's true
```

### println 函数

`println` 函数输出内容并在末尾添加换行符：

```ecl
// 输出并换行
println("Hello, World!");

// 输出多个值
println("First line");
println("Second line");
println("Third line");

// 输出变量值
var name = "Alice";
var age = 25;
println(name);
println(age);
```

### 字符串连接输出

```ecl
// 使用字符串连接输出复杂信息
var name = "Bob";
var score = 95;
print("Student: ");
print(name);
print(", Score: ");
print(score);
println("");

// 或者使用类型转换
print("Student: " + name + ", Score: " + <str>score);
println("");

// 使用转义序列进行格式化输出
println("Name\tScore\tGrade");           // 使用制表符对齐
println("Alice\t95\tA");
println("Bob\t87\tB");

// 多行输出
println("First Line\nSecond Line\nThird Line");  // 使用换行符

// 包含引号的输出
println("She said \"Hello World!\"");
println("Path: C:\\Users\\Documents");           // 反斜杠需要转义
```

## 输入函数

### input 函数

`input` 函数用于从用户获取输入：

```ecl
// 基本输入
var name = "";
input("Enter your name: ", name);
println("Hello, " + name + "!");

// 获取数字输入
var age = "";
input("Enter your age: ", age);
var ageNum = <int>age;
print("You are ");
print(ageNum);
print(" years old");
println("");

// 获取浮点数输入
var height = "";
input("Enter your height (m): ", height);
var heightNum = <float>height;
print("Your height is ");
print(heightNum);
print(" meters");
println("");
```

### 输入验证

```ecl
// 验证数字输入
func getValidNumber(prompt) {
    var input = "";
    var valid = false;
    var number = 0;
    
    while (!valid) {
        input(prompt, input);
        // 简单验证（实际可能需要更复杂的检查）
        if (isValidNumber(input)) {
            number = <int>input;
            valid = true;
        } else {
            println("Invalid input. Please enter a number.");
        }
    }
    
    return number;
}

// 使用验证输入
var age = getValidNumber("Enter your age: ");
print("Your age is: ");
print(age);
println("");
```

## 格式化输出

### 基本格式化

```ecl
// 使用字符串连接进行格式化
var name = "Alice";
var age = 30;
var city = "New York";

print("Name: ");
print(name);
print(", Age: ");
print(age);
print(", City: ");
print(city);
println("");

// 或者一行输出
println("Name: " + name + ", Age: " + <str>age + ", City: " + city);
```

### 表格输出

```ecl
// 输出表格数据
println("ID  Name    Age  Score");
println("-----------------------");
println("1   Alice   20   95");
println("2   Bob     22   87");
println("3   Charlie 19   92");
```

### 对齐输出

```ecl
// 手动对齐输出
print("Name     : ");
print("Alice");
println("");
print("Age      : ");
print(25);
println("");
print("Score    : ");
print(95.5);
println("");
```

### 数组和列表元素输出

```ecl
// 正确的数组元素输出（需要类型转换）
var <int>scores[3] = {85, 92, 78};
println("First score: " + <str>scores[0]);
println("All scores: " + <str>scores[0] + ", " + <str>scores[1] + ", " + <str>scores[2]);

// 列表元素输出（需要类型转换）
var mixed = {1, "hello", true, 3.14};
println("First element: " + <str>mixed[0]);
println("String element: " + mixed[3]);  // 字符串不需要转换
```

## 文件输出（概念性）

目前 ECL 不直接支持文件输出，但可以通过以下方式模拟：

```ecl
// 将输出收集到字符串中
var output = "";

func appendOutput(text) {
    output = output + text;
}

// 使用自定义输出函数
appendOutput("Processing data...\n");
appendOutput("Result: 42\n");

// 最后一次性输出
println(output);
```

## 错误处理

### 输入错误处理

```ecl
// 处理无效输入
func safeInputNumber(prompt) {
    var inputStr = "";
    input(prompt, inputStr);
    
    // 验证输入是否为有效数字
    if (isValidNumber(inputStr)) {
        return <int>inputStr;
    } else {
        println("Invalid number entered. Using default value 0.");
        return 0;
    }
}

// 使用安全输入
var number = safeInputNumber("Enter a number: ");
print("You entered: ");
print(number);
println("");
```

### 输出错误处理

```ecl
// ECL 的输出函数通常不会失败，但可以添加验证
func safePrint(value) {
    // 在某些情况下可能需要验证
    if (value != "") {
        print(value);
    } else {
        print("(empty)");
    }
}

safePrint("Hello");
safePrint("");  // 输出: (empty)
```

## 实际应用示例

### 1. 简单计算器

```ecl
// 简单的交互式计算器
func simpleCalculator() {
    println("Simple Calculator");
    println("Enter two numbers and an operation");
    
    var num1Str = "";
    var num2Str = "";
    var operation = "";
    
    input("First number: ", num1Str);
    input("Second number: ", num2Str);
    input("Operation (+, -, *, /): ", operation);
    
    var num1 = <float>num1Str;
    var num2 = <float>num2Str;
    var result = 0.0;
    
    if (operation == "+") {
        result = num1 + num2;
    } else if (operation == "-") {
        result = num1 - num2;
    } else if (operation == "*") {
        result = num1 * num2;
    } else if (operation == "/") {
        if (num2 != 0) {
            result = num1 / num2;
        } else {
            println("Error: Division by zero");
            return;
        }
    } else {
        println("Invalid operation");
        return;
    }
    
    print("Result: ");
    print(result);
    println("");
}

// simpleCalculator();
```

### 2. 用户信息收集

```ecl
// 收集用户信息
func collectUserInfo() {
    println("User Information Form");
    println("--------------------");
    
    var name = "";
    var ageStr = "";
    var email = "";
    
    input("Name: ", name);
    input("Age: ", ageStr);
    input("Email: ", email);
    
    var age = <int>ageStr;
    
    println("");
    println("Collected Information:");
    println("--------------------");
    println("Name: " + name);
    print("Age: ");
    print(age);
    println("");
    println("Email: " + email);
}

// collectUserInfo();
```

### 3. 成绩报告

```ecl
// 输出学生成绩报告
func printReportCard(name, math, science, english) {
    var total = math + science + english;
    var average = total / 3;
    
    println("REPORT CARD");
    println("===========");
    println("Student: " + name);
    println("----------");
    print("Math: ");
    print(math);
    println("");
    print("Science: ");
    print(science);
    println("");
    print("English: ");
    print(english);
    println("");
    println("----------");
    print("Total: ");
    print(total);
    println("");
    print("Average: ");
    print(average);
    println("");
    
    if (average >= 90) {
        println("Grade: A - Excellent!");
    } else if (average >= 80) {
        println("Grade: B - Good job!");
    } else if (average >= 70) {
        println("Grade: C - Satisfactory");
    } else {
        println("Grade: D - Needs improvement");
    }
}

// 使用示例
printReportCard("Alice", 95, 87, 92);
```

## 最佳实践

### 1. 清晰的提示信息

```ecl
// 好的做法：提供清晰的提示
input("Please enter your full name: ", name);
input("Enter your age (1-120): ", age);

// 不好的做法：提示不明确
input("Name: ", name);
input("Age: ", age);
```

### 2. 一致的输出格式

```ecl
// 保持输出格式一致
println("=== MENU ===");
println("1. Option One");
println("2. Option Two");
println("3. Exit");
println("============");

// 而不是不一致的格式
println("MENU:");
println("1) Option 1");
println("2)  Option 2");
println("3.Exit");
```

### 3. 适当的换行

```ecl
// 合理使用换行提高可读性
println("Welcome to the program!");
println("");  // 空行
println("Please select an option:");
println("1. Create new item");
println("2. View existing items");
println("3. Exit");
println("");  // 空行
```

### 4. 错误信息输出

```ecl
// 提供有用的错误信息
if (input < 0) {
    println("Error: Negative values are not allowed.");
    println("Please enter a positive number.");
}
```

## 性能考虑

### 1. 批量输出

```ecl
// 避免频繁的小输出
// 不推荐
for i in 1..100 {
    print("Item ");
    print(i);
    println("");
}

// 推荐：构建完整输出后一次性打印
var output = "";
for i in 1..100 {
    output = output + "Item " + <str>i + "\n";
}
print(output);
```

### 2. 减少输入提示

```ecl
// 在循环中减少重复提示
println("Enter numbers (enter -1 to finish):");
var sum = 0;
var input = 0;

while (input != -1) {
    input("Number: ", input);  // 只需要一次提示
    if (input != -1) {
        sum = sum + input;
    }
}
```

## 交互设计

### 1. 菜单系统

```ecl
// 简单的菜单系统
func showMenu() {
    println("=== MAIN MENU ===");
    println("1. Create new record");
    println("2. View records");
    println("3. Delete record");
    println("4. Exit");
    println("=================");
    print("Select option (1-4): ");
}

func processMenu() {
    var choice = "";
    while (choice != "4") {
        showMenu();
        input("", choice);
        
        if (choice == "1") {
            println("Creating new record...");
        } else if (choice == "2") {
            println("Viewing records...");
        } else if (choice == "3") {
            println("Deleting record...");
        } else if (choice == "4") {
            println("Goodbye!");
        } else {
            println("Invalid option. Please try again.");
        }
        println("");
    }
}

// processMenu();
```

### 2. 进度指示

```ecl
// 简单的进度指示
func showProgress(current, total) {
    print("Progress: ");
    print(current);
    print("/");
    print(total);
    print(" (");
    print((current * 100) / total);
    print("%)");
    println("");
}

// 使用示例
for i in 1..11 {
    showProgress(i, 10);
}
```

## 调试输出

### 1. 调试信息

```ecl
// 在开发过程中添加调试输出
func complexCalculation(a, b, c) {
    print("DEBUG: Input values - a=");
    print(a);
    print(", b=");
    print(b);
    print(", c=");
    print(c);
    println("");
    
    var result = a * b + c;
    
    print("DEBUG: Calculation result=");
    print(result);
    println("");
    
    return result;
}
```

### 2. 条件调试

```ecl
// 可控制的调试输出
var DEBUG = true;

func debugPrint(message) {
    if (DEBUG) {
        print("[DEBUG] ");
        print(message);
        println("");
    }
}

// 使用调试输出
debugPrint("Starting program");
var x = 10;
debugPrint("x = " + <str>x);
debugPrint("Program finished");
```

## 总结

ECL 的输入输出系统提供了：

- **输出函数**：`print`（不换行）和 `println`（换行）
- **输入函数**：`input` 获取用户输入
- **基本格式化**：通过字符串连接实现
- **错误处理**：验证输入数据的有效性

通过合理使用这些功能，您可以创建交互式的 ECL 程序，与用户进行有效的信息交换。记住要提供清晰的提示信息，保持输出格式一致，并在必要时添加错误处理机制。