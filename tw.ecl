func test() {
    var x = 0;
    while (x < 3) {
        x = x + 1;
    }
    return x;
}
var result = test();
println("循环测试结果: ");
println(result);