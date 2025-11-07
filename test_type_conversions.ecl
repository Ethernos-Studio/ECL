// 测试类型转换

<int>num = 42;
<str>text = "Hello";

// int to str
<str>num_str = num;
println("num_str = " + num_str);

// str to int (valid)
<int>text_length = "123";
println("text_length = " + text_length);

// bool
<bool>flag = true;
<int>flag_int = flag;  // bool to int
println("flag_int = " + flag_int);

// float to int
<float>fval = 3.7;
<int>f_to_int = fval;
println("f_to_int = " + f_to_int);

// double to float
<double>dval = 2.5;
<float>d_to_float = dval;
println("d_to_float = " + d_to_float);
