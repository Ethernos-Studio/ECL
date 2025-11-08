// 测试显式类型转换
var s = "hello";
var i = 123;
var result = s + <str>i;  // 使用显式转换
println(result);

// 测试数字运算
var a = 42;
var b = 58;
var sum = a + b;
println("Sum: " + sum);

// 测试字符串拼接
var str1 = "Hello, ";
var str2 = "World!";
var greeting = str1 + str2;
println(greeting);