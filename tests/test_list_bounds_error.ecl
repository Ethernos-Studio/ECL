// 测试列表越界错误

var lst = {"a", "b", "c"};

println("访问正常元素:");
println("lst[0] = " + lst[0]);

// 触发列表越界错误
println("访问越界元素:");
println("lst[10] = " + lst[10]);