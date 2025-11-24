// 数学工具函数库
func square(x) {
    return x * x;
}

func cube(x) {
    return x * x * x;
}

func power(base, exponent) {
    var result = 1;
    for i in 1..(exponent + 1) {
        result = result * base;
    }
    return result;
}

// 导出一些常量
var PI = 3.14159;
var E = 2.71828;