// 测试if表达式（三元运算符风格）

var x = 10;
var y = 20;

// 基本if表达式
var max = if (x > y) x else y;
println("max = "); println(max);

// if表达式在函数调用中
println("min = "); println(if (x < y) x else y);

// if表达式嵌套
var result = if (x > 0) 
                if (y > 0) 
                    "both positive" 
                else 
                    "x positive, y not" 
             else 
                "x not positive";
println(result);

// if表达式在计算中
var abs_x = if (x >= 0) x else -x;
println("abs("); print(x); print(") = "); println(abs_x);
