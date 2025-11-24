// 测试ECL语法高亮和智能提示
println("=== ECL语言测试 ===");

// 测试变量声明
var <int>age = 25;
var <str>name = "张三";
var <bool>isStudent = true;

// 测试数组
var <int>numbers[5] = {1, 2, 3, 4, 5};
var <str>names[3] = {"Alice", "Bob", "Charlie"};

// 测试函数定义
func greet(person) {
    println("Hello, " + person + "!");
    return true;
}

// 测试表达式函数
expr add(a, b) {
    return a + b;
}

// 测试控制结构
if (age >= 18) {
    println(name + " 是成年人");
} else {
    println(name + " 是未成年人");
}

// 测试循环
for i in 1..5 {
    println("数字: " + <str>i);
}

// 测试while循环
var <int>count = 0;
while (count < 3) {
    println("计数: " + <str>count);
    count = count + 1;
}

// 测试字符串转义
var <str>message = "Hello\nWorld\tTab\"Quote\"\\Backslash";
var <str>unicode = "Unicode: \u{4E2D}\u{6587}\u{6D4B}\u{8BD5}";

// 测试类型转换
var <str>strNumber = <str>42;
var <int>intValue = <int>"123";

// 测试输入输出
print("请输入您的名字: ");
input "名字: ", name;
println("欢迎, " + name + "!");

// 测试数组索引
numbers[0] = 10;
println("第一个数字: " + <str>numbers[0]);

// 测试导入
import "math_utils.ecl";

// 测试函数调用
var <int>result = add(5, 3);
println("5 + 3 = " + <str>result);

println("=== 测试完成 ===");