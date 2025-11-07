expr x2(l a) {
    return a * 2;
}

println("Direct call:");
print((16)x2);
println("");

println("Via variable:");
var result = (16)x2;
print(result);
println("");