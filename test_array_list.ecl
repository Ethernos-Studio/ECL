// 测试数组和列表功能

// 创建一个整型数组，长度为5，初始化为0
var <int>arr[5] = {0};
println("创建数组 arr[5] = {0}:");
println("arr[0] = " + arr[0]);
println("arr[1] = " + arr[1]);
println("arr[2] = " + arr[2]);
println("arr[3] = " + arr[3]);
println("arr[4] = " + arr[4]);

// 修改数组元素
arr[0] = 10;
arr[1] = 20;
arr[2] = 30;
println("修改后的数组:");
println("arr[0] = " + arr[0]);
println("arr[1] = " + arr[1]);
println("arr[2] = " + arr[2]);

// 创建一个列表
var lst = [];
println("创建空列表 lst = []");

// 创建一个带初始值的列表
var lst2 = {1, 2, 3, "hello", true};
println("创建带初始值的列表 lst2 = {1, 2, 3, \"hello\", true}:");
println("lst2[0] = " + lst2[0]);
println("lst2[1] = " + lst2[1]);
println("lst2[2] = " + lst2[2]);
println("lst2[3] = " + lst2[3]);
println("lst2[4] = " + lst2[4]);

// 修改列表元素
lst2[0] = 100;
lst2[3] = "world";
println("修改后的列表:");
println("lst2[0] = " + lst2[0]);
println("lst2[3] = " + lst2[3]);

// 测试单个值初始化数组
var <int>arr2[3] = {5};
println("用单个值初始化数组 arr2[3] = {5}:");
println("arr2[0] = " + arr2[0]);
println("arr2[1] = " + arr2[1]);
println("arr2[2] = " + arr2[2]);