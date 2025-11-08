// 创建一个整型数组，长度为5，初始化为1
var <int>arr[5] = {1};

// 正确的赋值 - 整数
arr[0] = 42;
println("成功将整数42赋值给arr[0]");

// 正确的赋值 - 浮点数（会自动转换为整数）
arr[1] = 3.14;
println("成功将浮点数3.14赋值给arr[1]（自动转换为3）");

// 正确的赋值 - 字符串数字（会自动转换为整数）
arr[2] = "100";
println("成功将字符串\"100\"赋值给arr[2]（自动转换为100）");

// 正确的赋值 - 布尔值（会自动转换为整数）
arr[3] = true;
println("成功将布尔值true赋值给arr[3]（自动转换为1）");

println("数组元素的值:");
println("arr[0] = " + arr[0]);
println("arr[1] = " + arr[1]);
println("arr[2] = " + arr[2]);
println("arr[3] = " + arr[3]);
println("arr[4] = " + arr[4]);