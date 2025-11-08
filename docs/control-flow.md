# 控制流

控制流语句允许程序根据条件执行不同的代码路径，或重复执行某些代码块。

## 条件语句

### if 语句

`if` 语句用于根据条件执行代码：

```ecl
// 基本 if 语句
var age = 18;
if (age >= 18) {
    println("You are an adult");
}

// if-else 语句
if (age >= 18) {
    println("You are an adult");
} else {
    println("You are a minor");
}

// if-else if-else 链
var score = 85;
if (score >= 90) {
    println("Grade: A");
} else if (score >= 80) {
    println("Grade: B");
} else if (score >= 70) {
    println("Grade: C");
} else {
    println("Grade: D");
}
```

### 条件表达式

条件表达式返回布尔值：

```ecl
var x = 10;
var y = 20;

// 比较运算符
if (x == y) { println("Equal"); }
if (x != y) { println("Not equal"); }
if (x < y) { println("x is less than y"); }
if (x > y) { println("x is greater than y"); }
if (x <= y) { println("x is less than or equal to y"); }
if (x >= y) { println("x is greater than or equal to y"); }

// 逻辑运算符（使用数值表示）
if (x > 0 && y > 0) {  // 在ECL中，非零值为真
    println("Both are positive");
}
```

### 嵌套条件

```ecl
var temperature = 25;
var isRaining = false;

if (temperature > 20) {
    if (!isRaining) {
        println("Good weather for outdoor activities");
    } else {
        println("Warm but raining");
    }
} else {
    println("Too cold");
}
```

## 循环语句

### for 循环

`for` 循环用于遍历范围：

```ecl
// 基本 for 循环
for i in 1..5 {
    print("Number: ");
    print(i);
    println("");
}

// 从 0 开始的循环
for i in 0..3 {
    println(i);
}

// 使用变量作为范围
var start = 2;
var end = 8;
for i in start..end {
    print("Value: ");
    print(i);
    println("");
}

// 复杂范围表达式
for i in (2+1)..(5*2) {
    print("i = ");
    print(i);
    println("");
}
```

### while 循环

`while` 循环在条件为真时重复执行：

```ecl
// 基本 while 循环
var counter = 0;
while (counter < 5) {
    print("Counter: ");
    print(counter);
    println("");
    counter = counter + 1;
}

// 使用 while 实现条件循环
var input = 0;
while (input != -1) {
    input("Enter a number (-1 to quit): ", input);
    if (input != -1) {
        print("You entered: ");
        print(input);
        println("");
    }
}

// 无限循环（需要有退出条件）
var flag = true;
var count = 0;
while (flag) {
    print("Count: ");
    print(count);
    println("");
    count = count + 1;
    if (count >= 3) {
        flag = false;
    }
}
```

### 循环中的控制语句

ECL 目前不支持 `break` 和 `continue` 语句，但可以通过条件判断实现类似功能：

```ecl
// 模拟 break
var i = 0;
while (i < 10) {
    if (i == 5) {
        // 模拟 break：通过修改条件变量
        i = 10;  // 使循环条件变为假
    } else {
        print(i);
        println("");
        i = i + 1;
    }
}

// 模拟 continue
var j = 0;
while (j < 5) {
    if (j == 2) {
        j = j + 1;  // 跳过当前迭代
    } else {
        print("j = ");
        print(j);
        println("");
        j = j + 1;
    }
}
```

## 循环与数组/列表

### 遍历数组

```ecl
// 遍历数组索引
var <int>numbers[4] = {10, 20, 30, 40};
for i in 0..4 {
    print("numbers[");
    print(i);
    print("] = ");
    print(numbers[i]);
    println("");
}

// 使用数组长度（逻辑上）
var size = 4;  // 假设我们知道数组大小
for i in 0..size {
    var value = numbers[i];
    print("Value: ");
    print(value);
    println("");
}
```

### 遍历列表

```ecl
// 遍历列表
var items = {1, "hello", true, 3.14};
var length = 4;  // 假设我们知道列表长度
for i in 0..length {
    print("Item ");
    print(i);
    print(": ");
    print(items[i]);
    println("");
}
```

## 嵌套循环

```ecl
// 乘法表
for i in 1..6 {
    for j in 1..6 {
        var result = i * j;
        print(i);
        print(" x ");
        print(j);
        print(" = ");
        print(result);
        print("  ");
    }
    println("");
}

// 二维数组模拟
var rows = 3;
var cols = 3;
for i in 0..rows {
    for j in 0..cols {
        var value = i * cols + j;
        print(value);
        print(" ");
    }
    println("");
}
```

## 控制流最佳实践

### 1. 避免深层嵌套

```ecl
// 不推荐：深层嵌套
if (condition1) {
    if (condition2) {
        if (condition3) {
            // 深层代码
            println("All conditions met");
        }
    }
}

// 推荐：提前返回或条件合并
if (condition1 && condition2 && condition3) {
    println("All conditions met");
}

// 或者使用否定条件提前处理
if (!condition1) {
    println("Condition 1 failed");
    return;
}
if (!condition2) {
    println("Condition 2 failed");
    return;
}
if (!condition3) {
    println("Condition 3 failed");
    return;
}
println("All conditions met");
```

### 2. 循环优化

```ecl
// 避免在循环中进行重复计算
var base = 10;
var limit = 100;

// 不推荐：循环内重复计算
for i in 1..10 {
    var result = base * i + limit * 2;  // limit*2 每次都计算
    print(result);
    println("");
}

// 推荐：提前计算
var precalculated = limit * 2;
for i in 1..10 {
    var result = base * i + precalculated;
    print(result);
    println("");
}
```

### 3. 确保循环终止

```ecl
// 确保循环变量会改变
var i = 0;
while (i < 10) {
    print(i);
    println("");
    i = i + 1;  // 确保 i 会递增
}

// 使用 for 循环更安全
for i in 0..10 {
    print(i);
    println("");
}
```

## 实际应用示例

### 搜索算法

```ecl
// 在数组中搜索元素
func findElement(arr, target) {
    var size = 5;  // 假设数组大小
    for i in 0..size {
        if (arr[i] == target) {
            return i;  // 返回找到的索引
        }
    }
    return -1;  // 未找到
}

var <int>numbers[5] = {1, 5, 3, 9, 2};
var index = findElement(numbers, 9);
if (index != -1) {
    print("Found at index: ");
    print(index);
    println("");
} else {
    println("Not found");
}
```

### 排序算法（冒泡排序）

```ecl
// 简单的冒泡排序实现
func bubbleSort(arr) {
    var size = 5;  // 假设数组大小
    for i in 0..size {
        for j in 0..(size-1) {
            if (arr[j] > arr[j+1]) {
                // 交换元素（需要临时变量）
                var temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
    return arr;
}

var <int>nums[5] = {64, 34, 25, 12, 22};
// bubbleSort(nums);  // 注意：这可能不工作，因为数组传递
```

### 输入验证

```ecl
// 验证输入直到有效
var validInput = false;
var number = 0;

while (!validInput) {
    input("Enter a positive number: ", number);
    if (number > 0) {
        validInput = true;
        print("Valid number entered: ");
        print(number);
        println("");
    } else {
        println("Invalid input. Please enter a positive number.");
    }
}
```

## 错误处理和边界情况

### 1. 空范围处理

```ecl
var start = 5;
var end = 3;
// 这种情况下循环不会执行
for i in start..end {
    print("This won't print");
    println("");
}
```

### 2. 零次循环

```ecl
// 当范围为空时
for i in 0..0 {
    print("This won't execute");
    println("");
}
```

### 3. 大范围循环

```ecl
// 大范围循环可能需要考虑性能
// for i in 1..1000000 {  // 可能执行很慢
//     // 处理
// }
```

## 总结

ECL 的控制流特性包括：

- **条件语句**：`if`, `if-else`, `if-else if-else`
- **循环语句**：`for` 循环（范围迭代）, `while` 循环（条件循环）
- **灵活的条件表达式**：支持比较和逻辑运算
- **嵌套控制流**：条件和循环可以嵌套使用

控制流语句是编程的基础，它们使程序能够根据不同的条件和重复执行不同的逻辑。在使用时要注意避免深层嵌套，确保循环能够正常终止，并考虑性能影响。