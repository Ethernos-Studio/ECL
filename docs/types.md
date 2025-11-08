# 数据类型

ECL 提供了丰富的数据类型系统，包括基本类型和复合类型。

## 基本类型

### 整数类型 (int)

用于表示整数值：

```ecl
var age = 25;
var negative = -10;
var zero = 0;
var large = 1000000;

// 显式声明类型
var <int>count = 100;
```

### 浮点数类型 (float)

用于表示单精度浮点数：

```ecl
var price = 19.99;
var pi = 3.14159;
var scientific = 1.5e-3;  // 科学计数法

// 显式声明类型
var <float>temperature = 98.6;
```

### 双精度浮点数类型 (double)

用于表示双精度浮点数：

```ecl
var precise = 3.141592653589793;
var largeDecimal = 123456789.123456789;

// 显式声明类型
var <double>accuracy = 0.0000001;
```

### 字符串类型 (str)

用于表示文本数据：

```ecl
var name = "Alice";
var greeting = "Hello, World!";
var empty = "";  // 空字符串

// 显式声明类型
var <str>message = "Welcome to ECL";
```

### 布尔类型 (bool)

用于表示逻辑值：

```ecl
var isActive = true;
var isComplete = false;

// 显式声明类型
var <bool>isEnabled = true;
```

## 复合类型

### 数组 (array)

固定长度的同类型数据集合：

```ecl
// 声明整型数组，长度为5，初始化为0
var <int>numbers[5] = {0};

// 声明并初始化数组
var <int>scores[3] = {85, 92, 78};

// 声明字符串数组
var <str>names[2] = {"Alice", "Bob"};

// 访问数组元素
var firstScore = scores[0];  // 85
var secondName = names[1];   // "Bob"

// 修改数组元素
scores[0] = 95;
names[1] = "Charlie";
```

### 列表 (list)

动态长度的异类型数据集合：

```ecl
// 创建空列表
var items = [];

// 创建带初始值的列表
var mixed = {1, "hello", true, 3.14};

// 访问列表元素
var first = mixed[0];   // 1 (int)
var second = mixed[1];  // "hello" (str)

// 修改列表元素
mixed[0] = 100;
mixed[1] = "world";
```

## 类型声明

ECL 支持静态类型声明，可以在变量声明时指定类型：

```ecl
// 基本类型声明
var <int>age = 25;
var <str>name = "Alice";
var <bool>isActive = true;
var <float>price = 19.99;
var <double>precision = 3.141592653589793;
```

## 类型推断

当没有显式声明类型时，ECL 会根据初始值自动推断类型：

```ecl
var x = 10;        // 推断为 int
var y = 3.14;      // 推断为 double
var z = "hello";   // 推断为 str
var flag = true;   // 推断为 bool
```

## 类型转换

ECL 支持显式类型转换：

```ecl
// 数字到字符串
var num = 123;
var str = <str>num;  // "123"

// 字符串到数字
var text = "456";
var value = <int>text;  // 456

// 数字类型之间转换
var integer = 10;
var floating = <float>integer;  // 10.0
var doubleVal = <double>floating;  // 10.0

// 布尔转换
var truth = true;
var numFromBool = <int>truth;  // 1
```

## 类型系统特性

### 类型安全

ECL 在运行时进行类型检查，确保类型安全：

```ecl
var <int>age = 25;
// age = "twenty-five";  // 运行时错误：类型不匹配
```

### 数组类型安全

数组元素必须与声明的类型一致：

```ecl
var <int>scores[3] = {85, 92, 78};
// scores[0] = "A";  // 运行时错误：不能将字符串赋值给整型数组
```

### 列表灵活性

列表可以包含不同类型的元素：

```ecl
var data = {1, "hello", true, 3.14};  // 合法：列表支持异类型
```

## 类型操作

### 获取类型信息

```ecl
// 在运行时可以通过值获取类型信息
var x = 10;
// x 的类型是 int

var y = "hello";
// y 的类型是 str
```

### 类型检查

虽然 ECL 没有内置的类型检查函数，但类型错误会在运行时被捕获：

```ecl
var <int>number = 10;
// 尝试赋值不兼容的类型会报错
// number = "string";  // 运行时类型错误
```

## 最佳实践

1. **明确类型声明**：对于重要变量，建议显式声明类型以提高代码可读性
2. **合理使用数组和列表**：
   - 当元素类型相同时使用数组
   - 当需要存储不同类型数据时使用列表
3. **注意类型转换**：类型转换可能导致数据丢失或精度降低
4. **避免类型错误**：确保赋值操作符合类型要求

```ecl
// 推荐：明确的类型声明
var <int>studentCount = 30;
var <str>className = "Mathematics";
var <bool>isExam = true;

// 推荐：合理的数据结构选择
var <int>scores[5] = {0};        // 同类型数据用数组
var studentInfo = {"Alice", 20, true};  // 异类型数据用列表
```

## 类型系统限制

1. **不支持自定义类型**：目前不支持结构体、枚举等自定义类型
2. **不支持泛型**：数组和列表不支持泛型参数
3. **运行时类型检查**：类型检查在运行时进行，而非编译时
4. **有限的类型推断**：仅在声明时根据初始值推断类型

未来版本可能会扩展类型系统，添加更多高级特性。