# 数据结构

ECL 提供了两种主要的数据结构：数组和列表，用于存储和操作集合数据。

## 数组 (Array)

数组是固定长度的同类型数据集合。

### 数组声明

```ecl
// 声明整型数组，长度为5，初始化为0
var <int>numbers[5] = {0};

// 声明并初始化数组
var <int>scores[3] = {85, 92, 78};

// 声明字符串数组
var <str>names[2] = {"Alice", "Bob"};

// 声明浮点数组
var <float>temperatures[4] = {98.6, 99.1, 97.8, 98.2};

// 声明布尔数组
var <bool>flags[3] = {true, false, true};
```

### 数组初始化方式

#### 1. 单值初始化

```ecl
// 用单个值初始化整个数组
var <int>zeros[5] = {0};      // [0, 0, 0, 0, 0]
var <int>ones[3] = {1};       // [1, 1, 1]
var <str>emptyStrings[4] = {""}; // ["", "", "", ""]
```

#### 2. 多值初始化

```ecl
// 用多个值初始化数组
var <int>numbers[5] = {1, 2, 3, 4, 5};
var <str>colors[3] = {"red", "green", "blue"};
var <float>values[4] = {1.1, 2.2, 3.3, 4.4};
```

#### 3. 混合初始化

```ecl
// 部分指定值，其余用默认值填充
var <int>mixed[5] = {10, 20};  // [10, 20, 0, 0, 0]
var <str>partial[4] = {"first", "second"};  // ["first", "second", "", ""]
```

### 数组访问

```ecl
var <int>scores[3] = {85, 92, 78};

// 访问数组元素
var firstScore = scores[0];   // 85
var secondScore = scores[1];  // 92
var thirdScore = scores[2];   // 78

// 打印数组元素
print("First score: ");
print(scores[0]);
println("");

// 正确的数组元素输出（需要类型转换）
println("First score: " + <str>scores[0]);
println("All scores: " + <str>scores[0] + ", " + <str>scores[1] + ", " + <str>scores[2]);

// 使用循环遍历数组
for i in 0..3 {
    print("Score ");
    print(i);
    print(": ");
    print(scores[i]);
    println("");
}
```

### 数组修改

```ecl
var <int>numbers[3] = {1, 2, 3};

// 修改数组元素
numbers[0] = 10;
numbers[1] = 20;
numbers[2] = 30;

// 基于其他元素的值修改
numbers[0] = numbers[1] + numbers[2];  // 20 + 30 = 50
```

### 数组遍历

```ecl
var <int>values[4] = {10, 20, 30, 40};

// 使用 for 循环遍历数组
for i in 0..4 {
    print("values[");
    print(i);
    print("] = ");
    print(values[i]);
    println("");
}

// 计算数组元素之和
var sum = 0;
for i in 0..4 {
    sum = sum + values[i];
}
print("Sum: ");
print(sum);
println("");
```

### 数组限制

1. **固定长度**：数组一旦声明，长度不能改变
2. **同类型**：数组中所有元素必须是相同类型
3. **索引从0开始**：第一个元素的索引是0

## 列表 (List)

列表是动态长度的异类型数据集合。

### 列表声明

```ecl
// 创建空列表
var items = [];

// 创建带初始值的列表
var mixed = {1, "hello", true, 3.14};

// 创建同类型列表
var numbers = {1, 2, 3, 4, 5};
var names = {"Alice", "Bob", "Charlie"};
var flags = {true, false, true, false};
```

### 列表访问

```ecl
var data = {1, "hello", true, 3.14};

// 访问列表元素
var first = data[0];   // 1 (int)
var second = data[1];  // "hello" (str)
var third = data[2];   // true (bool)
var fourth = data[3];  // 3.14 (double)

// 打印列表元素
print("Second element: ");
print(data[1]);
println("");
```

### 列表修改

```ecl
var items = {1, "hello", true};

// 修改列表元素
items[0] = 100;
items[1] = "world";
items[2] = false;

// 修改为不同类型（列表支持异类型）
items[0] = "new string";
items[1] = 3.14;
items[2] = 42;
```

### 列表遍历

```ecl
var data = {10, "hello", true, 3.14, "world"};

// 假设我们知道列表长度
var length = 5;

// 遍历列表
for i in 0..length {
    print("Element ");
    print(i);
    print(": ");
    print(data[i]);
    println("");
}
```

### 列表特性

1. **动态长度**：理论上可以动态改变（但当前ECL实现可能有限制）
2. **异类型**：列表中可以存储不同类型的元素
3. **索引从0开始**：第一个元素的索引是0

## 数组与列表对比

| 特性 | 数组 | 列表 |
|------|------|------|
| 长度 | 固定 | 动态（理论上） |
| 类型 | 同类型 | 异类型 |
| 内存 | 连续分配 | 可能非连续 |
| 性能 | 访问速度快 | 访问速度相对较慢 |
| 灵活性 | 低 | 高 |
| 类型安全 | 高 | 低 |

### 选择建议

```ecl
// 使用数组的场景：同类型数据，长度固定
var <int>scores[5] = {0};        // 学生成绩
var <str>weekdays[7] = {"Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"};
var <bool>flags[10] = {false};   // 布尔标志数组

// 使用列表的场景：异类型数据，灵活性要求高
var studentInfo = {"Alice", 20, true, 3.8};  // 姓名, 年龄, 是否在校, GPA
var mixedData = {1, "hello", true, 3.14, "world"};
```

## 实际应用示例

### 1. 数学计算

```ecl
// 使用数组存储数学序列
var <int>fibonacci[10] = {0};

// 初始化斐波那契数列
fibonacci[0] = 0;
fibonacci[1] = 1;
for i in 2..10 {
    fibonacci[i] = fibonacci[i-1] + fibonacci[i-2];
}

// 打印斐波那契数列
for i in 0..10 {
    print("Fibonacci[");
    print(i);
    print("] = ");
    print(fibonacci[i]);
    println("");
}
```

### 2. 数据处理

```ecl
// 使用列表存储记录
var studentRecords = {
    {"Alice", 85, "A"},
    {"Bob", 92, "A"},
    {"Charlie", 78, "B"}
};

// 处理学生记录（假设支持二维访问）
// 注意：当前ECL可能不直接支持这种语法
```

### 3. 游戏开发

```ecl
// 使用数组表示游戏棋盘
var <int>board[9] = {0};  // 井字棋棋盘

// 初始化棋盘
for i in 0..9 {
    board[i] = 0;  // 0表示空位
}

// 设置棋子
board[4] = 1;  // 中心位置放置玩家1的棋子

// 检查获胜条件（简化版）
func checkWin(board) {
    // 检查行
    if (board[0] != 0 && board[0] == board[1] && board[1] == board[2]) {
        return board[0];
    }
    // 更多检查...
    return 0;
}
```

## 错误处理

### 数组越界访问

```ecl
var <int>numbers[3] = {1, 2, 3};

// 安全访问数组元素
func safeArrayAccess(arr, index) {
    // 假设数组长度为3
    var length = 3;
    if (index >= 0 && index < length) {
        return arr[index];
    } else {
        print("Error: Index out of bounds");
        println("");
        return 0;
    }
}

var value = safeArrayAccess(numbers, 5);  // 错误处理
```

### 类型错误

```ecl
var <int>scores[3] = {85, 92, 78};

// 安全赋值
func safeArrayAssignment(arr, index, value) {
    // 检查类型（简化示例）
    // 在实际中，ECL会在运行时检查类型
    arr[index] = value;
    return arr;
}

// scores[0] = "A";  // 运行时类型错误
```

## 最佳实践

### 1. 数组使用

```ecl
// 好的做法：明确数组用途
var <int>studentScores[30] = {0};  // 30个学生的成绩
var <str>monthNames[12] = {"January", "February", "March", "April", "May", "June", 
                           "July", "August", "September", "October", "November", "December"};

// 初始化数组
for i in 0..30 {
    studentScores[i] = 0;
}
```

### 2. 列表使用

```ecl
// 好的做法：合理使用列表存储异构数据
var person = {"Alice", 25, true, "Engineer"};  // 姓名, 年龄, 是否在职, 职业

// 处理列表数据
func processPerson(person) {
    var name = person[0];
    var age = person[1];
    var isEmployed = person[2];
    var job = person[3];
    
    print("Name: ");
    print(name);
    println("");
    print("Age: ");
    print(age);
    println("");
    // 更多处理...
}
```

### 3. 性能考虑

```ecl
// 对于频繁访问的数据，使用数组
var <int>frequentlyAccessed[1000] = {0};

// 对于需要灵活存储的数据，使用列表
var flexibleData = {1, "string", true, 3.14};

// 避免在循环中重复创建数组/列表
// 不好的做法：
for i in 0..100 {
    var temp[10] = {0};  // 每次循环都创建新数组
    // 处理...
}

// 好的做法：
var temp[10] = {0};  // 在循环外创建
for i in 0..100 {
    // 重置数组
    for j in 0..10 {
        temp[j] = 0;
    }
    // 处理...
}
```

### 4. 内存管理

```ecl
// 合理选择数组大小
var <int>smallArray[10] = {0};   // 小数组
var <int>largeArray[1000] = {0}; // 大数组，注意内存使用

// 对于不确定大小的数据，使用列表
var dynamicData = [];
```

## 高级用法

### 1. 多维数组模拟

```ecl
// 使用一维数组模拟二维数组
// 3x3 矩阵
var <int>matrix[9] = {0};

// 访问 matrix[row][col] 转换为 matrix[row * 3 + col]
func getMatrixElement(matrix, row, col) {
    return matrix[row * 3 + col];
}

func setMatrixElement(matrix, row, col, value) {
    matrix[row * 3 + col] = value;
}

// 使用示例
setMatrixElement(matrix, 1, 1, 5);  // 设置中心元素为5
var center = getMatrixElement(matrix, 1, 1);  // 获取中心元素
```

### 2. 数组作为函数参数

```ecl
// 处理数组的函数
func sumArray(arr) {
    var sum = 0;
    // 假设数组长度为5
    for i in 0..5 {
        sum = sum + arr[i];
    }
    return sum;
}

var <int>numbers[5] = {1, 2, 3, 4, 5};
var total = sumArray(numbers);
```

## 总结

ECL 的数据结构系统提供了：

- **数组**：固定长度、同类型、高效访问
- **列表**：动态长度、异类型、灵活存储

选择合适的数据结构对于编写高效的 ECL 程序至关重要。数组适合处理同类型、固定大小的数据集合，而列表适合需要存储不同类型数据或需要灵活性的场景。

通过合理使用这些数据结构，您可以有效地组织和处理程序中的数据。