// 测试类型错误

<int>age = 25;

// 这将导致类型错误：不能将字符串赋值给int变量
// <int>invalid = "not a number";

<str>name = "Bob";

// 这将导致类型错误：不能将数字赋值给str变量（需要显式转换）
// <str>invalid_str = 123;

// 测试有效的重新赋值
age = 30;  // 同类型赋值，应该工作
name = "Charlie";  // 同类型赋值，应该工作

println("age = " + age);
println("name = " + name);
