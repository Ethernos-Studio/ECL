// 测试数组越界错误

var <int>arr[3] = {1, 2, 3};
var lst = {"a", "b", "c"};

println("访问正常元素:");
println("arr[0] = " + arr[0]);
println("lst[0] = " + lst[0]);

// 触发数组越界错误
println("访问越界元素:");
println("arr[10] = " + arr[10]);