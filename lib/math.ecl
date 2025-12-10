// ECL数学库 - 高级数学函数
import "std.ecl"

// 平方根函数 - 使用牛顿迭代法
func sqrt(x) {
    if x < 0 {
        return 0;  // 负数返回0
    }
    if x == 0 {
        return 0;
    }
    
    var guess = x / 2;
    var counter = 0;
    while counter < 20 {  // 最多迭代20次
        var new_guess = (guess + x / guess) / 2;
        if abs(new_guess - guess) < 0.000001 {
            return new_guess;
        }
        guess = new_guess;
        counter = counter + 1;
    }
    return guess;
}

// 幂函数 - 支持整数指数
func power(base, exponent) {
    if exponent == 0 {
        return 1;
    }
    if exponent < 0 {
        return 0;  // 负指数返回0
    }
    
    var result = 1;
    var counter = 0;
    while counter < exponent {
        result = result * base;
        counter = counter + 1;
    }
    return result;
}

// 阶乘函数
func factorial(n) {
    if n <= 1 {
        return 1;
    }
    var result = 1;
    var counter = 2;
    while counter <= n {
        result = result * counter;
        counter = counter + 1;
    }
    return result;
}

// 判断是否为质数
func is_prime(n) {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    var counter = 5;
    while counter * counter <= n {
        if n % counter == 0 || n % (counter + 2) == 0 {
            return false;
        }
        counter = counter + 6;
    }
    return true;
}

// 计算圆的面积
func circle_area(radius) {
    return PI * radius * radius;
}

// 计算圆的周长
func circle_circumference(radius) {
    return 2 * PI * radius;
}

// 计算两点之间的距离
func distance(x1, y1, x2, y2) {
    var dx = x2 - x1;
    var dy = y2 - y1;
    return sqrt(dx * dx + dy * dy);
}