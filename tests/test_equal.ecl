// 测试 == 运算符
var a = 5;
var b = 5;
var c = 10;

println("测试相等性:");
println("5 == 5: " + <str>(a == b));
println("5 == 10: " + <str>(a == c));

// 在if语句中使用 ==
if (a == b) {
    println("a 和 b 相等");
} else {
    println("a 和 b 不相等");
}

if (a == c) {
    println("a 和 c 相等");
} else {
    println("a 和 c 不相等");
}