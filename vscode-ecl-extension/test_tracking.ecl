// 测试变量、函数、表达式追踪功能

// 函数定义 - 应该显示为蓝色（定义）
func calculateSum(a, b) {
    return a + b;
}

// 表达式函数定义 - 应该显示为蓝色（定义）
expr multiply(x, y) {
    return x * y;
}

// 变量定义 - 应该显示为蓝色（定义）
var <int>result = 0;
var <str>message = "计算结果：";

// 函数和变量引用 - 应该显示为不同的颜色
println("调用calculateSum函数：");
result = calculateSum(5, 3);  // calculateSum是函数调用，result是变量引用
println(message + <str>result);  // message和result都是变量引用

println("调用multiply表达式：");
var <int>product = (2)multiply;  // multiply是表达式调用
println("乘积：" + <str>product);  // product是变量引用

// 测试循环变量
for i in 1..5 {  // i是变量定义
    var <int>temp = i * 2;  // temp是变量定义，i是变量引用
    println("temp = " + <str>temp);  // temp是变量引用
}

// 测试条件语句
var <bool>condition = true;  // condition是变量定义
if (condition) {  // condition是变量引用
    var <str>status = "条件为真";  // status是变量定义
    println(status);  // status是变量引用
} else {
    var <str>status = "条件为假";  // status是变量定义
    println(status);  // status是变量引用
}

// 测试嵌套函数调用
func outerFunction(value) {
    var <int>innerVar = value * 2;  // innerVar是变量定义，value是参数引用
    return innerVar;  // innerVar是变量引用
}

var <int>finalResult = outerFunction(10);  // outerFunction是函数调用，finalResult是变量定义
println("最终结果：" + <str>finalResult);  // finalResult是变量引用