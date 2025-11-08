// ECL超级示例 - 包含所有特性和改进的错误处理

// 1. 基本变量声明和赋值
var <int>age = 25;
var <str>name = "Alice";
var <bool>isStudent = true;
var <float>height = 5.8;
var <double>weight = 65.5;

// 2. 数组和列表
var <int>numbers[5] = {1, 2, 3, 4, 5};
var mixedList = {"Hello", 42, true, 3.14};

// 3. 基本运算
var sum = 10 + 5;
var difference = 20 - 8;
var product = 6 * 7;
var quotient = 15 / 3;
var isEqual = (10 == 10);
var isGreater = (15 > 10);

// 4. 字符串连接
var greeting = "Hello, " + name + "!";
println(greeting);

// 5. 条件语句
if (age >= 18) {
    println("成年人");
} else {
    println("未成年人");
}

// 6. 循环语句
for i in 1..5 {
    println("循环: " + i);
}

// 7. while循环
var counter = 0;
while (counter < 3) {
    println("While循环: " + counter);
    counter = counter + 1;
}

// 8. 函数定义和调用
func calculateArea(length, width) {
    return length * width;
}

var area = calculateArea(10, 5);
println("面积: " + area);

// 9. 表达式函数
expr add(l a, r b) {
    return a + b;
}

expr multiply(l a, r b) {
    return a * b;
}

// 使用表达式函数 - 中缀语法
var result1 = 5 add 3;
var result2 = 4 multiply 6;

// 使用表达式函数 - 括号语法
var result3 = (7)add(2);
var result4 = (3)multiply(9);

println("5 add 3 = " + result1);
println("4 multiply 6 = " + result2);
println("7 add 2 = " + result3);
println("3 multiply 9 = " + result4);

// 10. 数组操作
println("数组第一个元素: " + numbers[0]);
numbers[0] = 100;
println("修改后数组第一个元素: " + numbers[0]);

// 11. 列表操作
println("列表第一个元素: " + mixedList[0]);
mixedList[1] = 99;
println("修改后列表第二个元素: " + mixedList[1]);

// 12. if表达式
var max = if (10 > 5) 10 else 5;
println("最大值: " + max);

// 13. 复杂表达式
var complex = ((10 + 5) * 2) / 3;
println("复杂表达式结果: " + complex);

// 14. 字符串和数字混合操作
var strNum = "结果是: " + (5 add 3);
println(strNum);

// 15. 嵌套函数调用
var nested = calculateArea((2 add 3), (4 multiply 2));
println("嵌套调用结果: " + nested);

// 16. 新增的列表操作函数
println("=== 列表操作函数 ===");
var testList = {"a", "b", "c"};
println("初始列表长度: " + len(testList));

append(testList, "d");
println("添加元素后列表长度: " + len(testList));
println("添加后的列表: " + testList[0] + ", " + testList[1] + ", " + testList[2] + ", " + testList[3]);

var popped = pop(testList);
println("弹出的元素: " + popped);
println("弹出后列表长度: " + len(testList));

// 17. 所有变量输出
println("=== 变量输出 ===");
println("姓名: " + name);
println("年龄: " + age);
println("是否学生: " + isStudent);
println("身高: " + height);
println("体重: " + weight);
println("数组: " + numbers[0] + ", " + numbers[1] + ", " + numbers[2]);
println("列表: " + mixedList[0] + ", " + mixedList[1] + ", " + mixedList[2]);

// 18. 最后的运行时错误 - 数组越界访问
// 这将触发我们改进的错误处理机制
println("访问不存在的数组元素:");
println("numbers[10] = " + numbers[10]);