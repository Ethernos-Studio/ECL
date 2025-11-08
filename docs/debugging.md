# 调试与错误处理

ECL 提供了完善的错误处理机制和调试工具，帮助开发者识别和解决代码中的问题。

## 错误类型

### 1. 词法错误 (Lexical Errors)

词法错误发生在词法分析阶段，通常是由于无效字符或不完整的标记。

```ecl
// 错误示例
var x = 10.  // Unexpected dot - 应该是 10.0 或 10
var y = 10@5  // Unexpected character '@'
```

错误信息：
```
Unexpected character: '.' at line 1, column 12
```

### 2. 语法错误 (Syntax Errors)

语法错误发生在语法分析阶段，代码结构不符合 ECL 语法规则。

```ecl
// 错误示例
var x = ;  // Variable declaration requires an initializer
if (x > 5  // Missing closing parenthesis
func test(  // Missing closing parenthesis
{ var x = 10  // Missing closing brace
```

错误信息：
```
Syntax error: unexpected semicolon, please check if the previous statement is complete
  --> test.ecl:1:9
  |
1 | var x = ;
  |         ^
  = help: syntax is `var name = value`
  = example: var name = "Alice"
```

### 3. 运行时错误 (Runtime Errors)

运行时错误在程序执行期间发生，包括未定义标识符、类型错误等。

```ecl
// 未定义标识符
print(undefinedVariable);  // Undefined identifier: "undefinedVariable"

// 类型错误
var <int>arr[3] = {1, 2, 3};
arr[0] = "string";  // Type error in array assignment

// 数组越界
var <int>numbers[3] = {1, 2, 3};
print(numbers[5]);  // Index out of bounds
```

## 错误信息详解

### 错误消息结构

ECL 的错误消息遵循标准格式：

```
错误描述
  --> 文件路径:行号:列号
  |
行号 | 源代码行
  |   ^ 指示错误位置
  = 帮助信息（可选）
  = 示例（可选）
```

### 常见错误消息

#### 变量相关错误

```ecl
// 未初始化变量声明
var x;  // Variable declaration requires an initializer

错误信息：
Variable declaration requires an initializer
  --> test.ecl:1:7
  |
1 | var x;
  |       ^
  = help: syntax is `var name = value`
  = example: var name = "Alice"
```

#### 函数相关错误

```ecl
// 函数参数不匹配
func add(a, b) {
    return a + b;
}
var result = add(1, 2, 3);  // Function 'add' expects 2 arguments, got 3

错误信息：
Function 'add' expects 2 arguments, got 3
```

#### 数组相关错误

```ecl
// 数组越界
var <int>arr[3] = {1, 2, 3};
print(arr[5]);  // Index out of bounds

错误信息：
Index out of bounds: index 5 is out of range for array of length 3
  --> test.ecl:2:7
  |
2 | print(arr[5]);
  |       ^^^^^^^
  = help: make sure the index is within the valid range [0, length)
  = example: if arr has 5 elements, valid indices are 0, 1, 2, 3, 4
```

## 调试工具

### 1. 词法分析器调试

ECL 提供了词法分析器调试模式：

```bash
# 调试字符串
cargo run -- --debug-lexer "var x = 10 + 5;"

# 调试文件
cargo run -- --debug-lexer tests/debug_lexer.ecl
```

调试输出示例：
```
Debugging lexer for input: 'var x = 10 + 5;'
Position: (1, 1), Token: Var
Position: (1, 5), Token: Identifier("x")
Position: (1, 7), Token: Equal
Position: (1, 9), Token: Number(10.0)
Position: (1, 12), Token: Plus
Position: (1, 14), Token: Number(5.0)
Position: (1, 15), Token: Semicolon
Position: (1, 16), Token: Eof
```

### 2. 打印调试

使用 `print` 和 `println` 函数进行调试输出：

```ecl
func complexCalculation(a, b, c) {
    print("DEBUG: Input values - a=");
    print(a);
    print(", b=");
    print(b);
    print(", c=");
    print(c);
    println("");
    
    var result = a * b + c;
    
    print("DEBUG: Intermediate result=");
    print(result);
    println("");
    
    return result;
}
```

### 3. 条件调试

使用条件语句控制调试输出：

```ecl
var DEBUG = true;

func debugPrint(message) {
    if (DEBUG) {
        print("[DEBUG] ");
        print(message);
        println("");
    }
}

// 使用调试输出
debugPrint("Starting calculation");
var x = 10;
debugPrint("x = " + <str>x);
var result = x * 2;
debugPrint("Result = " + <str>result);
```

## 调试技巧

### 1. 分段调试

将复杂代码分解为小段进行调试：

```ecl
// 而不是一次性写完整个函数
func processComplexData(data) {
    // 复杂的处理逻辑...
}

// 分段调试
func processComplexData(data) {
    // 第一步：验证输入
    print("Step 1: Validating input");
    if (data == null) {
        print("Error: Null input");
        return;
    }
    
    // 第二步：预处理
    print("Step 2: Preprocessing");
    var processed = preprocess(data);
    print("Preprocessed data: " + processed);
    
    // 第三步：计算
    print("Step 3: Calculating");
    var result = calculate(processed);
    print("Result: " + result);
    
    return result;
}
```

### 2. 边界条件测试

测试边界条件以发现潜在错误：

```ecl
// 测试数组边界
func testArrayBounds() {
    var <int>arr[3] = {1, 2, 3};
    
    // 测试有效索引
    print("Valid access arr[0]: " + arr[0]);
    print("Valid access arr[2]: " + arr[2]);
    
    // 测试边界索引
    print("Boundary access arr[3]: ");  // 应该出错
    print(arr[3]);
}
```

### 3. 类型验证

验证类型转换和赋值操作：

```ecl
// 验证类型转换
func testTypeConversion() {
    var str = "123";
    var num = <int>str;
    print("String '123' to int: " + num);
    
    var invalidStr = "not_a_number";
    print("Trying to convert 'not_a_number': ");
    // var invalidNum = <int>invalidStr;  // 应该出错
}
```

## 错误处理策略

### 1. 预防性检查

在关键操作前进行检查：

```ecl
func safeArrayAccess(arr, index) {
    // 假设数组长度为5
    var length = 5;
    if (index >= 0 && index < length) {
        return arr[index];
    } else {
        print("Error: Index " + index + " out of bounds for array of length " + length);
        println("");
        return 0;  // 返回默认值
    }
}

var <int>numbers[3] = {1, 2, 3};
var value = safeArrayAccess(numbers, 5);  // 安全访问
```

### 2. 错误恢复

设计错误恢复机制：

```ecl
func robustCalculation(a, b) {
    // 检查除零错误
    if (b == 0) {
        print("Warning: Division by zero, returning 0");
        println("");
        return 0;
    }
    
    return a / b;
}

var result = robustCalculation(10, 0);  // 不会崩溃
```

### 3. 输入验证

验证用户输入以防止错误：

```ecl
func getValidInput(prompt) {
    var input = "";
    var valid = false;
    
    while (!valid) {
        input(prompt, input);
        
        // 简单验证
        if (isValidNumber(input)) {
            valid = true;
        } else {
            println("Invalid input. Please enter a valid number.");
        }
    }
    
    return <int>input;
}

var number = getValidInput("Enter a number: ");
```

## 使用 REPL 调试

### 1. 交互式测试

在 REPL 中测试代码片段：

```ecl
ecl> // 测试函数
ecl> func add(a, b) {
...>     return a + b;
...> }
ecl> print(add(5, 3));
8
ecl> // 测试边界条件
ecl> print(add(0, 0));
0
ecl> print(add(-5, 5));
0
```

### 2. 逐步验证

逐步验证复杂逻辑：

```ecl
ecl> // 验证循环逻辑
ecl> var sum = 0;
ecl> for i in 1..6 {
...>     sum = sum + i;
...>     print("i=" + i + ", sum=" + sum + "\n");
...> }
i=1, sum=1
i=2, sum=3
i=3, sum=6
i=4, sum=10
i=5, sum=15
```

## 日志记录模式

### 1. 简单日志

```ecl
var LOG_LEVEL = 2;  // 0=错误, 1=警告, 2=信息, 3=调试

func log(level, message) {
    if (level <= LOG_LEVEL) {
        var levelStr = "";
        if (level == 0) {
            levelStr = "ERROR";
        } else if (level == 1) {
            levelStr = "WARN";
        } else if (level == 2) {
            levelStr = "INFO";
        } else {
            levelStr = "DEBUG";
        }
        
        print("[" + levelStr + "] " + message);
        println("");
    }
}

// 使用日志
log(2, "Application started");
log(0, "Critical error occurred");
```

### 2. 函数执行跟踪

```ecl
var TRACE = true;

func traceEnter(funcName) {
    if (TRACE) {
        print(">>> Entering " + funcName);
        println("");
    }
}

func traceExit(funcName) {
    if (TRACE) {
        print("<<< Exiting " + funcName);
        println("");
    }
}

func tracedFunction(x) {
    traceEnter("tracedFunction");
    var result = x * 2;
    traceExit("tracedFunction");
    return result;
}
```

## 常见调试场景

### 1. 循环调试

```ecl
// 调试循环变量
for i in 1..6 {
    print("DEBUG: Loop iteration i=" + i);
    println("");
    
    // 循环体逻辑
    var result = i * i;
    print("Result: " + result);
    println("");
}
```

### 2. 递归调试

```ecl
// 调试递归函数
func factorialWithDebug(n) {
    print("DEBUG: factorial(" + n + ")");
    println("");
    
    if (n <= 1) {
        print("DEBUG: Base case, returning 1");
        println("");
        return 1;
    }
    
    var result = n * factorialWithDebug(n - 1);
    print("DEBUG: factorial(" + n + ") = " + result);
    println("");
    return result;
}

var fact = factorialWithDebug(5);
```

### 3. 数组操作调试

```ecl
// 调试数组操作
func processArrayWithDebug(arr) {
    print("DEBUG: Processing array of length 3");
    println("");
    
    for i in 0..3 {
        print("DEBUG: Processing element [" + i + "] = " + arr[i]);
        println("");
        
        arr[i] = arr[i] * 2;
        print("DEBUG: Updated element [" + i + "] = " + arr[i]);
        println("");
    }
    
    return arr;
}

var <int>numbers[3] = {1, 2, 3};
processArrayWithDebug(numbers);
```

## 性能调试

### 1. 简单性能测试

```ecl
// 测试代码执行时间
func timeExecution(codeDescription) {
    print("Starting: " + codeDescription);
    println("");
    
    // 这里执行要测试的代码
    var start = getCurrentTime();  // 假设有此函数
    
    // 执行代码...
    for i in 1..1000 {
        var x = i * i;
    }
    
    var end = getCurrentTime();  // 假设有此函数
    var duration = end - start;
    
    print("Completed: " + codeDescription + " in " + duration + " units");
    println("");
}

// timeExecution("Loop calculation");
```

### 2. 内存使用监控

```ecl
// 监控内存使用（概念性）
func monitorMemory(operation) {
    var before = getMemoryUsage();  // 假设有此函数
    
    // 执行操作
    operation();
    
    var after = getMemoryUsage();
    var diff = after - before;
    
    print("Memory change for " + operation + ": " + diff + " bytes");
    println("");
}
```

## 最佳实践

### 1. 错误信息友好性

```ecl
// 好的错误信息
if (divisor == 0) {
    print("Error: Cannot divide by zero. Please provide a non-zero divisor.");
    println("");
    return 0;
}

// 不好的错误信息
if (divisor == 0) {
    print("Error: div/0");
    return 0;
}
```

### 2. 调试代码的管理

```ecl
// 使用全局开关控制调试输出
var DEBUG = false;

func debug(message) {
    if (DEBUG) {
        print("[DEBUG] " + message);
        println("");
    }
}

// 在发布版本中设置 DEBUG = false
// 在开发版本中设置 DEBUG = true
```

### 3. 一致的错误处理

```ecl
// 在整个项目中保持一致的错误处理方式
func handleError(errorType, details) {
    print("[" + errorType + "] " + details);
    println("");
    
    // 根据错误类型采取不同行动
    if (errorType == "FATAL") {
        exit();  // 假设有退出函数
    }
}

// 使用一致的错误处理
if (fileNotFound) {
    handleError("ERROR", "Required file not found: config.txt");
}
```

## 总结

ECL 的调试和错误处理系统提供了：

- **详细的错误信息**：包含位置、帮助和示例
- **多种调试工具**：词法分析器调试、打印调试等
- **错误分类**：词法错误、语法错误、运行时错误
- **预防性措施**：输入验证、边界检查等
- **REPL 调试支持**：交互式测试和验证

通过合理使用这些工具和技巧，您可以有效地识别和解决 ECL 程序中的问题，提高代码质量和开发效率。