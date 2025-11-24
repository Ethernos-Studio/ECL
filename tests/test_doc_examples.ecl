// 测试文档中的示例代码
println("=== 测试文档示例 ===");

// 测试语法文档中的转义示例
var greeting = "Hello, World!";
var newline = "Line 1\nLine 2\nLine 3";
var tab = "Name\tAge\tCity";
var quote = "She said \"Hello World!\"";
var backslash = "Path: C:\\Users\\Documents";

println("基本字符串: " + greeting);
println("换行测试: " + newline);
println("制表符测试: " + tab);
println("引号测试: " + quote);
println("反斜杠测试: " + backslash);

// 测试Unicode转义
var emoji = "Smile: \u{1F600}";
var chinese = "中文: \u{4E2D}\u{6587}\u{6D4B}\u{8BD5}";
println("Emoji测试: " + emoji);
println("中文测试: " + chinese);

// 测试数组索引修复（来自ditui.ecl）
var <int>a[10] = {0};
a[0] = 1;
a[1] = 1;
for i in 2..10 {
    a[i] = a[i-1] + a[i-2];
}
println("斐波那契数列第10个元素: " + <str>a[9]);

println("=== 所有测试完成 ===");