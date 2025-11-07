expr x2(l a) {
    return a * 2;
}

expr negate(r a) {
    return a * -1;
}

expr multiply(l a, r b) {
    return a * b;
}

println("Test 1: (16)x2");
print((16)x2);
println("");

println("Test 2: negate 16");
print(negate 16);
println("");

println("Test 3: (2)multiply(8)");
print((2)multiply(8));
println("");