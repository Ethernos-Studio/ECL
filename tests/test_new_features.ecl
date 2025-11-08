// 测试数组和列表的新功能

// 1. 创建数组和列表
var <int>arr[3] = {1, 2, 3};
var lst = {"a", "b", "c"};

println("初始数组: " + arr[0] + ", " + arr[1] + ", " + arr[2]);
println("初始列表: " + lst[0] + ", " + lst[1] + ", " + lst[2]);

// 2. 测试len函数
var arr_len = len(arr);
var lst_len = len(lst);
println("数组长度: " + arr_len);
println("列表长度: " + lst_len);

// 3. 测试append函数
append(lst, "d");
println("添加元素后的列表: " + lst[0] + ", " + lst[1] + ", " + lst[2] + ", " + lst[3]);
println("新列表长度: " + len(lst));

// 4. 测试pop函数
var popped = pop(lst);
println("弹出的元素: " + popped);
println("弹出元素后的列表: " + lst[0] + ", " + lst[1] + ", " + lst[2]);
println("新列表长度: " + len(lst));

// 5. 测试数组越界访问（应该报错）
// println("访问不存在的元素: " + arr[10]);

// 6. 测试数组越界赋值（应该报错）
// arr[10] = 5;

// 7. 测试列表越界访问（应该报错）
// println("访问不存在的元素: " + lst[10]);

println("所有测试完成！");