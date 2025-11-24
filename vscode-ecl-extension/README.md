# ECL Language Support for VSCode

为 ECL 编程语言提供语法高亮、智能提示、符号追踪、定义跳转和跨文件import支持的 VSCode 扩展。

## 功能特性

### 🔍 语法高亮
- 关键字高亮（`var`, `func`, `if`, `for`, `while` 等）
- 数据类型高亮（`int`, `str`, `bool`, `float`, `double`）
- **函数和变量定义与引用区分高亮**：
  - 函数定义：蓝色加粗（`entity.name.function.declaration.ecl`）
  - 函数调用：淡黄色（`entity.name.function.call.ecl`）
  - 变量定义：浅蓝色加粗（`variable.other.declaration.ecl`）
  - 变量引用：浅蓝色（`variable.other.reference.ecl`）
- 字符串和数字常量高亮
- 注释高亮（`//` 单行注释和 `/* */` 多行注释）
- 转义序列高亮（`\n`, `\t`, `\"`, `\u{XXXX}` 等）

### 💡 智能提示
- 关键字自动补全
- 数据类型自动补全
- 内置函数提示（`print`, `println`, `input`）
- 上下文相关的智能补全

### 📋 代码片段
- 函数声明模板
- 变量声明模板
- 控制结构模板（`if`, `for`, `while`）
- 表达式模板

### 🔧 其他功能
- **定义跳转支持**：Ctrl+点击函数或变量名可跳转到定义处
- **跨文件import支持**：支持跳转到import文件中的函数和变量定义
  - 在工作区根目录查找import文件
  - 在文件所在目录查找import文件
  - 自动处理.ecl扩展名
- **查找所有引用**：支持查找函数和变量的所有引用位置
- 悬停文档提示
- 文档符号导航
- 括号匹配和自动闭合
- 代码折叠支持
- 自定义ECL主题配色

## 支持的语法特性

### 基本语法
```ecl
// 变量声明
var <int>age = 25;
var name = "John";

// 函数定义
func greet(person) {
    println("Hello, " + person);
    return true;
}

// 表达式函数
expr add(a, b) {
    return a + b;
}
```

### 控制结构
```ecl
// 条件语句
if (age > 18) {
    println("Adult");
} else {
    println("Minor");
}

// 循环语句
for i in 1..10 {
    println("Number: " + <str>i);
}

while (condition) {
    // loop body
}
```

### 数组和类型转换
```ecl
// 数组声明
var <int>numbers[5] = {1, 2, 3, 4, 5};

// 类型转换
var strNumber = <str>42;
var intValue = <int>"123";
```

### 字符串和转义
```ecl
var message = "Hello\nWorld";
var unicode = "Emoji: \u{1F600}";
var quote = "Say \"Hello\"";
```

## 安装方法

1. 下载扩展包
2. 在 VSCode 中打开扩展视图（Ctrl+Shift+X）
3. 点击"..."菜单，选择"从VSIX安装"
4. 选择下载的 `.vsix` 文件

## 开发说明

### 项目结构
```
vscode-ecl-extension/
├── package.json              # 扩展配置
├── tsconfig.json            # TypeScript 配置
├── language-configuration.json # 语言配置
├── syntaxes/
│   └── ecl.tmLanguage.json  # 语法高亮规则
├── snippets/
│   └── ecl.json            # 代码片段
└── src/
    └── extension.ts        # 扩展主程序
```

### 构建步骤
```bash
npm install
npm run compile
```

### 打包扩展
```bash
npm install -g vsce
vsce package
```

## 更新日志

### 0.3.0
- 新增跨文件import支持
- 支持跳转到import文件中的符号定义
- 在工作区根目录和文件所在目录查找import文件
- 自动处理import文件路径和扩展名
- 增强跨文件符号追踪能力

### 0.2.0
- 新增符号追踪功能
- 支持定义跳转（Ctrl+点击）
- 支持查找所有引用
- 优化语法高亮，区分定义和引用
- 添加自定义ECL主题配色
- 增强变量和函数识别能力

### 0.1.0
- 初始版本
- 基础语法高亮
- 智能代码补全
- 代码片段支持
- 悬停文档提示

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个扩展。

## 许可证

MIT License