func test() {
    println("Starting test");
    var x = 10;
    println(x);
    println(undefined_var);  // 这行应该报错，显示正确的行号和列号
    println("This should not be printed");
}

test();
