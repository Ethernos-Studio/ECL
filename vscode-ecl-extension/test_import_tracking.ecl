// 测试import文件的符号追踪功能

// 导入数学工具库
import "math_utils";

// 测试从import文件导入的函数 - 应该能够跳转到math_utils.ecl中的定义
var result1 = square(5);     // square函数应该能够跳转到math_utils.ecl
var result2 = cube(3);       // cube函数应该能够跳转到math_utils.ecl  
var result3 = power(2, 8);   // power函数应该能够跳转到math_utils.ecl

// 测试从import文件导入的常量 - 应该能够跳转到math_utils.ecl中的定义
var pi_value = PI;           // PI常量应该能够跳转到math_utils.ecl
var e_value = E;             // E常量应该能够跳转到math_utils.ecl

// 测试本地函数 - 应该能够正常跳转
func localFunction(x) {
    return x * 2;
}

var localResult = localFunction(10);  // 应该能够跳转到上面的localFunction定义

// 测试混合使用
var finalResult = square(localResult) + PI;  // square跳转到import文件，localResult和PI分别跳转到各自的定义

println("测试结果：");
println("square(5) = " + <str>result1);
println("cube(3) = " + <str>result2);
println("power(2, 8) = " + <str>result3);
println("PI = " + <str>pi_value);
println("E = " + <str>e_value);
println("localFunction(10) = " + <str>localResult);
println("finalResult = " + <str>finalResult);