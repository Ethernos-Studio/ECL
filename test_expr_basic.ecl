// 测试基本的expr功能

expr x2(l a) {
    return a * 2;
}

// 测试调用
print("Test 1: ");
print((16)x2);
println("");

expr negate(r a) {
    return a * -1;
}

print("Test 2: ");
print(negate 16);
println("");

expr multiply(l a, r b) {
    return a * b;
}

print("Test 3: ");
print((2)multiply(8));
println("");
