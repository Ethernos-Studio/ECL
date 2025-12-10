// ECL标准库 - 数学函数
func add(a, b) {
    return a + b;
}

func multiply(a, b) {
    return a * b;
}

func abs(x) {
    if x < 0 {
        return -x;
    } else {
        return x;
    }
}

func max(a, b) {
    if a > b {
        return a;
    } else {
        return b;
    }
}

func min(a, b) {
    if a < b {
        return a;
    } else {
        return b;
    }
}

// 常量
var PI = 3.14159265359;
var E = 2.71828182846;