// 测试带有多个值的列表初始化

// 创建一个带初始值的列表
var lst = {1, 2, 3};
println("创建带初始值的列表:");
println("lst[0] = " + <str>lst[0]);
println("lst[1] = " + <str>lst[1]);
println("lst[2] = " + <str>lst[2]);

// 修改列表元素
lst[0] = 100;
lst[1] = 200;
println("修改后的列表:");
println("lst[0] = " + <str>lst[0]);
println("lst[1] = " + <str>lst[1]);
println("lst[2] = " + <str>lst[2]);

// 测试不同类型混合的列表
var mixed = {10, 20, 30};
println("创建混合类型列表:");
println("mixed[0] = " + <str>mixed[0]);
println("mixed[1] = " + <str>mixed[1]);
println("mixed[2] = " + <str>mixed[2]);