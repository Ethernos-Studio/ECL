var i = 1;
var j = 1;

for i in 1..10 {
    for j in 1..(i+1) {
        print(j);
        print("x");
        print(i);
        print("=");
        print(i*j)
        print("  ")
    }
    println("")
}