// 测试严格类型检查 - 这应该会报错
var s = "hello";
var i = 123;
var result = s + i;  // 不同类型间的运算应该报错
println(result);