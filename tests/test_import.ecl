// 测试import功能
println("=== 测试import功能 ===");

// 导入数学工具库
import "math_utils";

// 测试导入的函数
var result1 = square(5);
println("square(5) = " + <str>result1);

var result2 = cube(3);
println("cube(3) = " + <str>result2);

var result3 = power(2, 8);
println("power(2, 8) = " + <str>result3);

// 测试导入的常量
println("PI = " + <str>PI);
println("E = " + <str>E);

println("=== import功能测试完成 ===");