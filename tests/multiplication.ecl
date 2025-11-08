var i = 1;
var j = 1;

for i in 1..10 {
    for j in 1..(i+1) {
        var result = i * j;
        print(j)
        print("x")
        print(i)
        print("=")
        print(result);
        print(" ")
    }
    println("");
}