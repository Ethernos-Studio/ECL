// 测试更多类型转换用例
var a = 123;
var b = <str>a;
var c = <float>a;
println("a = " + <str>a);
println("b = " + b);
println("c = " + <str>c);

// 测试比较操作
var x = 5 < 10;
var y = 10 > 5;
var z = 5 <= 5;
println("5 < 10 = " + <str>x);
println("10 > 5 = " + <str>y);
println("5 <= 5 = " + <str>z);

// 测试复杂表达式
var result = (<str>42) + " is the answer";
println(result);