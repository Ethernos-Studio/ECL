# 类型转换

ECL 支持显式类型转换，允许在不同数据类型之间进行转换。

## 类型转换语法

使用 `<type>value` 语法进行类型转换：

```ecl
// 基本类型转换语法
var newValue = <targetType>originalValue;
```

## 数字类型转换

### 整数转换

```ecl
// 整数到其他类型
var intVal = 123;
var floatFromInt = <float>intVal;    // 123.0
var doubleFromInt = <double>intVal;  // 123.0
var strFromInt = <str>intVal;        // "123"
var boolFromInt = <bool>intVal;      // true (非零为真)

// 整数转换示例
var age = 25;
var ageStr = <str>age;       // "25"
var ageFloat = <float>age;   // 25.0
```

### 浮点数转换

```ecl
// 浮点数到其他类型
var floatVal = 3.14;
var intFromFloat = <int>floatVal;    // 3 (截断小数部分)
var doubleFromFloat = <double>floatVal; // 3.14
var strFromFloat = <str>floatVal;    // "3.14"
var boolFromFloat = <bool>floatVal;  // true (非零为真)

// 浮点数转换示例
var price = 19.99;
var priceInt = <int>price;   // 19 (截断)
var priceStr = <str>price;   // "19.99"
```

### 双精度浮点数转换

```ecl
// 双精度浮点数到其他类型
var doubleVal = 3.141592653589793;
var intFromDouble = <int>doubleVal;  // 3 (截断小数部分)
var floatFromDouble = <float>doubleVal; // 3.141593 (精度降低)
var strFromDouble = <str>doubleVal;  // "3.141592653589793"
var boolFromDouble = <bool>doubleVal; // true (非零为真)
```

## 字符串类型转换

### 字符串到数字

```ecl
// 字符串到整数
var strInt = "123";
var intFromStr = <int>strInt;    // 123

// 字符串到浮点数
var strFloat = "3.14";
var floatFromStr = <float>strFloat;  // 3.14

// 字符串到双精度浮点数
var strDouble = "3.141592653589793";
var doubleFromStr = <double>strDouble; // 3.141592653589793

// 字符串到布尔值
var strTrue = "true";
var boolFromStr = <bool>strTrue;  // true
var strOne = "1";
var boolFromOne = <bool>strOne;   // true
var strFalse = "false";
var boolFromFalseStr = <bool>strFalse; // false
var strZero = "0";
var boolFromZero = <bool>strZero; // false
```

### 字符串到其他类型

```ecl
// 字符串到字符串（无变化）
var text = "Hello";
var sameText = <str>text;  // "Hello"

// 数字到字符串
var number = 42;
var numberStr = <str>number;  // "42"
var pi = 3.14159;
var piStr = <str>pi;  // "3.14159"
```

## 布尔类型转换

### 布尔值到其他类型

```ecl
// 布尔值到整数
var trueVal = true;
var intFromTrue = <int>trueVal;    // 1
var falseVal = false;
var intFromFalse = <int>falseVal;  // 0

// 布尔值到浮点数
var floatFromTrue = <float>trueVal;   // 1.0
var floatFromFalse = <float>falseVal; // 0.0

// 布尔值到双精度浮点数
var doubleFromTrue = <double>trueVal;   // 1.0
var doubleFromFalse = <double>falseVal; // 0.0

// 布尔值到字符串
var strFromTrue = <str>trueVal;   // "true"
var strFromFalse = <str>falseVal; // "false"
```

## 数组和列表转换

### 数组元素类型转换

```ecl
// 数组元素类型转换
var <int>intArray[3] = {1, 2, 3};

// 将数组元素转换为字符串（需要逐个转换）
var <str>strArray[3] = {"", "", ""};
strArray[0] = <str>intArray[0];  // "1"
strArray[1] = <str>intArray[1];  // "2"
strArray[2] = <str>intArray[2];  // "3"
```

### 列表元素转换

```ecl
// 列表元素转换
var mixedList = {1, 2.5, "3", true};

// 转换列表元素（需要逐个转换）
var intFromFirst = <int>mixedList[0];    // 1
var strFromSecond = <str>mixedList[1];   // "2.5"
var intFromThird = <int>mixedList[2];    // 3
var strFromFourth = <str>mixedList[3];   // "true"
```

## 复杂转换示例

### 链式转换

```ecl
// 链式类型转换
var strNumber = "123.45";
var floatVal = <float>strNumber;     // 123.45
var intVal = <int>floatVal;          // 123
var strAgain = <str>intVal;          // "123"

// 一行完成多个转换
var result = <str>(<int>(<float>"456.78"));  // "456"
```

### 条件转换

```ecl
// 根据条件进行类型转换
func convertSafely(value, targetType) {
    // 简化示例，实际实现会更复杂
    if (targetType == "int") {
        return <int>value;
    } else if (targetType == "str") {
        return <str>value;
    }
    return value;
}

var number = 123;
var strNumber = convertSafely(number, "str");  // "123"
```

## 错误处理

### 转换失败处理

```ecl
// 字符串到数字转换失败处理
var invalidStr = "not_a_number";
// var result = <int>invalidStr;  // 运行时错误

// 安全转换函数
func safeStringToInt(str) {
    // 简化示例，实际需要更复杂的验证
    // 假设我们有一个验证函数
    if (canConvertToInt(str)) {
        return <int>str;
    } else {
        print("Conversion failed: ");
        print(str);
        println(" is not a valid integer");
        return 0;
    }
}

// 使用安全转换
var value = safeStringToInt("123");     // 123
var invalid = safeStringToInt("abc");   // 0 with error message
```

### 类型检查

```ecl
// 在转换前检查类型（概念性示例）
func isString(value) {
    // 简化实现
    return true;  // 假设所有值都可以是字符串
}

func isNumber(value) {
    // 简化实现
    return true;  // 假设所有值都可以是数字
}

// 安全转换示例
func convertIfPossible(value, targetType) {
    if (targetType == "int" && isNumber(value)) {
        return <int>value;
    } else if (targetType == "str") {
        return <str>value;
    }
    return value;
}
```

## 最佳实践

### 1. 明确转换意图

```ecl
// 好的做法：明确转换意图
var age = 25;
var ageStr = <str>age;  // 明确将年龄转换为字符串

// 不好的做法：隐式依赖类型转换
var result = age + " years old";  // 可能导致错误
```

### 2. 避免精度丢失

```ecl
// 注意精度丢失
var precise = 3.141592653589793;
var lessPrecise = <float>precise;  // 精度降低
var backToInt = <int>lessPrecise;  // 3 (小数部分丢失)

// 如果需要保持精度，避免不必要的转换
```

### 3. 验证输入

```ecl
// 在转换前验证输入
func validateAndConvert(str) {
    // 检查字符串是否为有效数字
    if (isValidNumber(str)) {
        return <int>str;
    } else {
        print("Invalid input: ");
        print(str);
        println("");
        return 0;
    }
}

// 使用验证函数
var userInput = "123";
var number = validateAndConvert(userInput);  // 123

var invalidInput = "abc";
var zero = validateAndConvert(invalidInput);  // 0 with error
```

### 4. 一致性使用

```ecl
// 在项目中保持类型转换的一致性
// 数字到字符串转换
var age = 25;
var ageStr = <str>age;

var score = 95.5;
var scoreStr = <str>score;

// 字符串到数字转换
var inputAge = "30";
var actualAge = <int>inputAge;

var inputScore = "87.5";
var actualScore = <float>inputScore;
```

## 性能考虑

### 1. 避免不必要的转换

```ecl
// 不必要的转换
var x = 10;
var y = <int>(<str>x);  // 从int转str再转int，无意义

// 直接使用
var z = x;  // 更高效
```

### 2. 批量转换优化

```ecl
// 批量转换数组元素
var <int>intArray[5] = {1, 2, 3, 4, 5};
var <str>strArray[5] = {"", "", "", "", ""};

// 一次性转换所有元素
for i in 0..5 {
    strArray[i] = <str>intArray[i];
}
```

## 实际应用示例

### 1. 用户输入处理

```ecl
// 处理用户输入
func processUserInput() {
    var ageInput = "";
    input("Enter your age: ", ageInput);
    
    // 安全转换年龄输入
    var age = <int>ageInput;
    
    var heightInput = "";
    input("Enter your height (cm): ", heightInput);
    
    // 安全转换身高输入
    var height = <float>heightInput;
    
    print("Age: ");
    print(age);
    print(", Height: ");
    print(height);
    println(" cm");
}
```

### 2. 数据格式化

```ecl
// 格式化输出数据
func formatStudentInfo(name, age, score) {
    var info = "Name: " + name + 
               ", Age: " + <str>age + 
               ", Score: " + <str>score;
    return info;
}

var studentInfo = formatStudentInfo("Alice", 20, 95.5);
println(studentInfo);
```

### 3. 数学计算

```ecl
// 混合类型数学计算
func calculateBMI(weightStr, heightStr) {
    // 转换输入字符串为数字
    var weight = <float>weightStr;  // 体重(kg)
    var height = <float>heightStr;  // 身高(m)
    
    // 计算BMI
    var bmi = weight / (height * height);
    
    // 返回字符串格式的结果
    return <str>bmi;
}

var bmi = calculateBMI("70", "1.75");  // "22.8571428571429"
```

## 限制和注意事项

### 1. 转换限制

```ecl
// 某些转换可能不支持或产生意外结果
var complexStr = "123abc";
// var result = <int>complexStr;  // 可能失败或只转换部分

var emptyStr = "";
// var zero = <int>emptyStr;  // 可能失败
```

### 2. 精度问题

```ecl
// 浮点数精度问题
var largeInt = 123456789;
var floatVal = <float>largeInt;  // 可能精度丢失
var backToInt = <int>floatVal;   // 可能不等于原始值
```

### 3. 布尔转换特殊情况

```ecl
// 布尔转换的特殊情况
var strVal = "false";
var boolVal = <bool>strVal;  // true (因为非空字符串为真)

// 正确的布尔字符串转换
func stringToBool(str) {
    if (str == "true" || str == "1") {
        return true;
    } else if (str == "false" || str == "0") {
        return false;
    }
    // 其他情况的处理
    return false;
}
```

## 总结

ECL 的类型转换系统提供了：

- **显式转换**：使用 `<type>value` 语法
- **基本类型支持**：int, float, double, str, bool 之间的转换
- **运行时检查**：转换失败会在运行时报告错误
- **灵活性**：支持链式转换和复杂转换场景

通过合理使用类型转换，您可以处理不同数据类型之间的交互，但要注意避免不必要的转换和潜在的精度丢失问题。始终验证输入数据的有效性，并在转换失败时提供适当的错误处理。