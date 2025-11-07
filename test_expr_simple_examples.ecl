expr x2(l a) {
    return a * 2;
}

expr negate(r a) {
    return a * -1;
}

expr x(l a, r b) {
    return a * b;
}

println("16 x2 = ");
println(16 x2);

println("(16)x2 = ");
println((16)x2);

println("negate 16 = ");
println(negate 16);

println("negate(16) = ");
println(negate(16));

println("2 x 16 = ");
println(2 x 16);

println("(2)x(16) = ");
println((2)x(16));