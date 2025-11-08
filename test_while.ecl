// 测试while循环

println("=== 测试while循环 ===");
var count = 0;
while (count < 5) {
    print("count = ");
    println(count);
    count = count + 1;
}

println("\n=== 测试while循环累加 ===");
var sum = 0;
var i = 1;
while (i <= 10) {
    sum = sum + i;
    i = i + 1;
}
println("1到10的和是: ");
println(sum);