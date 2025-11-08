// 测试带有多个值的列表初始化

// 创建一个带初始值的列表
var lst = {1, 2, 3};
println("创建带初始值的列表:");
println("lst[0] = " + lst[0]);
println("lst[1] = " + lst[1]);
println("lst[2] = " + lst[2]);

// 修改列表元素
lst[0] = 100;
lst[1] = 200;
println("修改后的列表:");
println("lst[0] = " + lst[0]);
println("lst[1] = " + lst[1]);
println("lst[2] = " + lst[2]);

// 测试不同类型混合的列表
var mixed = {10, 20, 30};
println("创建混合类型列表:");
println("mixed[0] = " + mixed[0]);
println("mixed[1] = " + mixed[1]);
println("mixed[2] = " + mixed[2]);