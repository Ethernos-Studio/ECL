// 测试Unicode转义序列
println("测试Unicode转义：");

// 使用\u{XXXX}格式
println("笑脸：\u{1F600} 心形：\u{2665} 星星：\u{2605}");

// 使用\uXXXX格式（4位十六进制）
println("ASCII字符：\u0041\u0042\u0043");  // ABC

// 中文字符
println("中文：\u{4E2D}\u{6587}\u{6D4B}\u{8BD5}");

// 混合转义
println("混合：Hello\u0020World\n新行：\u{1F600}\u0009Tab");

// 错误处理测试
println("无效Unicode：\u{ZZZZ} 和 \u{12345678}");
println("不完整格式：\u{123");

// 表情符号
println("表情：\u{1F601} \u{1F602} \u{1F603} \u{1F604}");