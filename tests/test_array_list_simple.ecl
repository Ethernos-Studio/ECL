// 测试数组和列表功能

// 创建一个整型数组，长度为5，初始化为0
var <int>arr[5] = {0};
println("创建数组 arr[5] = {0}:");
println("arr[0] = " + <str>arr[0]);
println("arr[1] = " + <str>arr[1]);
println("arr[2] = " + <str>arr[2]);
println("arr[3] = " + <str>arr[3]);
println("arr[4] = " + <str>arr[4]);

// 修改数组元素
arr[0] = 10;
arr[1] = 20;
arr[2] = 30;
println("修改后的数组:");
println("arr[0] = " + <str>arr[0]);
println("arr[1] = " + <str>arr[1]);
println("arr[2] = " + <str>arr[2]);

// 创建一个列表
var lst = [];
println("创建空列表 lst = []");

// 测试单个值初始化数组
var <int>arr2[3] = {5};
println("用单个值初始化数组 arr2[3] = {5}:");
println("arr2[0] = " + <str>arr2[0]);
println("arr2[1] = " + <str>arr2[1]);
println("arr2[2] = " + <str>arr2[2]);