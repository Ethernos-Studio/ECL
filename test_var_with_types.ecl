// 测试使用var关键字的类型声明

var <int>count = 10;
var <str>message = "Hello World";
var <bool>enabled = false;
var <float>ratio = 0.5;
var <double>precision = 0.123456789;

println("count = " + count);
println("message = " + message);
println("enabled = " + enabled);
println("ratio = " + ratio);
println("precision = " + precision);

// 测试重新赋值
count = 20;
message = "Updated";
enabled = true;

println("
After update:");
println("count = " + count);
println("message = " + message);
println("enabled = " + enabled);