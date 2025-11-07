// 调试expr执行

expr x2(l a) {
    println("Inside x2, a = ");
    println(a);
    return a * 2;
}

println("Calling x2 with 16:");
var result = (16)x2;
println("Result = ");
println(result);