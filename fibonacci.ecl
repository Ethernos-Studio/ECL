func fib(n) {
    var preans = 1;
    var ans = 1;
    var tmp = 1;
    var j = 1;
    for j in 1..n {
        tmp = ans;
        ans = preans + ans;
        preans = tmp;
    }
    return ans;
}

var i = 1;
for i in 1..21 {
print("第");print(i);print("项:");println(fib(i));
}
