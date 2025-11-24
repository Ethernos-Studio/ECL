// 测试环境变量功能
println("=== 测试环境变量功能 ===");

// 测试预定义的环境变量
println("PATH = " + env_PATH);
println("HOME = " + env_HOME);
println("USER = " + env_USER);
println("OS = " + env_OS);

// 测试访问不存在的变量（应该返回空字符串）
println("NON_EXISTENT = '" + env_NON_EXISTENT + "'");

// 测试在表达式中使用环境变量
var user_path = env_HOME + "/documents";
println("用户文档路径: " + user_path);

// 测试环境变量在条件语句中的使用
// 注意：字符串比较需要转换为数字或使用其他方法
// 这里我们简单地检查字符串长度
var os_len = length(env_OS);
if (os_len > 0) {
    println("检测到操作系统信息");
}

println("=== 环境变量功能测试完成 ===");