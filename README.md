# ECL(EthernosCommandLang)

ECL是一个基于Rust开发的半过程编程语言，用于教学和基本数学运算

核心特性：
`expr` 关键字，用于定义表达式

支持以下参数类型
```
l
r
l r
r1 r2
r1 argv
```

如
```
expr x2(l a) {
    return a*2;
}

print(16 x2); // 32
print((16)x2); // 同上，更推荐(优先级大于func)

expr negate(r a) {
    return a * -1;
}

print(negate 16); // -16
print(negate(16)); // 与func几乎相同

expr x(l a, r b) {
    return a * b;
}

print(2 x 16); // 32(注意不能打成2x16了，不认)
print((2)x(16)); // 32 (带括号的可以去掉空格，优先级大于func)

expr max(r1 a, r2 b) {
    if(a > b) return a;
    else b;
}

print(max 3 16) //16
print(max(3)(16)) //16

expr sort(r1 n, argv arr) {
    for (var i = 0:n-1; i++) {
        for (var j = 0:n-1; j++) {
            if (arr[j] > arr[j + 1]) {
            int temp = arr[j];
            arr[j] = arr[j + 1];
            arr[j + 1] = temp;
            }
        }
    }
}

var n = 7;
print(sort n (64 34 25 12 22 11 90))
print(sort(n)(64 34 25 12 22 11 90)) // {90, 64, 34, 25, 22, 12, 11} argv必须加括号
```

部分关键字：
```
var
if
const
expr
func
return
```