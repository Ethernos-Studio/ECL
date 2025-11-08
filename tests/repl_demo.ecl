// REPL演示文件
// 运行: cargo run
// 然后在REPL中输入这些命令:

var x = 10;
print("变量x的值是: ");
print(x);
println("");

for i in 1..5 {
    print("i = ");
    print(i);
    println("");
}

var sum = 0;
for j in 1..(x/2) {
    sum = sum + j;
}
print("1到");
print(x/2);
print("的和是: ");
print(sum);
println("");