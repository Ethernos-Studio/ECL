var <int>a[10] = {0};
a[0] = 1;
a[1] = 1;
for i in 2..10 {
    a[i] = a[i-1] + a[i-2];
    println("a[" + <str>i + "]=" + <str>a[i]);
}
print(a[9])