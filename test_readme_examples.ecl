// 测试README中的expr示例

expr x2(l a) {
    return a * 2;
}

print("16 x2 = ");
print(16 x2);
println("");

print("(16)x2 = ");
print((16)x2);
println("");

expr negate(r a) {
    return a * -1;
}

print("negate 16 = ");
print(negate 16);
println("");

print("negate(16) = ");
print(negate(16));
println("");

expr x(l a, r b) {
    return a * b;
}

print("2 x 16 = ");
print(2 x 16);
println("");

print("(2)x(16) = ");
print((2)x(16));
println("");

expr max(r1 a, r2 b) {
    if(a > b) return a;
    else return b;
}

print("max 3 16 = ");
print(max 3 16);
println("");

print("max(3)(16) = ");
print(max(3)(16));
println("");