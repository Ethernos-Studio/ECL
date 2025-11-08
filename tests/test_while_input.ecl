// 测试while循环和input语句

println("=== 测试while循环 ===");
var count = 0;
while (count < 5) {
    print("count = ");
    println(count);
    count = count + 1;
}

println("
=== 测试input语句 ===");
var name;
input("请输入你的名字: ", name);
println("你好, ");
print(name);
println("!");

var age;
input("请输入你的年龄: ", age);
println("你输入的年龄是: ");
println(age);

var num1;
var num2;
input("请输入第一个数字: ", num1);
input("请输入第二个数字: ", num2);
print("两个数字的和是: ");
println(num1 + num2);