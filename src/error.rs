use std::fmt;

// 集中管理所有错误消息
pub mod error_messages {
    // Lexer 错误
    pub fn unexpected_character(ch: char, line: usize, column: usize) -> String {
        format!("Unexpected character: '{}' at line {}, column {}", ch, line, column)
    }
    
    pub fn unexpected_dot() -> String {
        "Unexpected character: '.'. Did you mean '..' for range?".to_string()
    }
    
    // Parser 错误 - Unexpected tokens
    pub fn unexpected_semicolon() -> String {
        "Syntax error: unexpected semicolon, please check if the previous statement is complete".to_string()
    }
    
    pub fn unexpected_eof() -> String {
        "Syntax error: unexpected end of file, please check for unclosed brackets or statements".to_string()
    }
    
    pub fn unexpected_operator(op: &str) -> String {
        format!("Syntax error: unexpected {} sign, please check if the expression is complete", op)
    }
    
    pub fn unexpected_keyword(keyword: &str) -> String {
        format!("Syntax error: unexpected {} keyword, please check statement structure", keyword)
    }
    
    pub fn unexpected_var() -> String {
        "Variable declaration requires an initializer".to_string()
    }
    
    pub fn unexpected_loop_keyword(keyword: &str) -> String {
        format!("Syntax error: unexpected {} keyword, please check loop syntax", keyword)
    }
    
    pub fn unexpected_conditional_keyword(keyword: &str) -> String {
        format!("Syntax error: unexpected {} keyword, please check conditional statement syntax", keyword)
    }
    
    pub fn unexpected_else() -> String {
        "Syntax error: unexpected else keyword, please check for corresponding if statement".to_string()
    }
    
    pub fn unexpected_input() -> String {
        "Syntax error: unexpected input keyword, please check input statement syntax".to_string()
    }
    
    pub fn unexpected_func() -> String {
        "Syntax error: unexpected func keyword, please check function definition syntax".to_string()
    }
    
    pub fn unexpected_expr() -> String {
        "Syntax error: unexpected expr keyword, please check expression syntax".to_string()
    }
    
    pub fn unexpected_return() -> String {
        "Syntax error: unexpected return keyword, please check function return syntax".to_string()
    }
    
    pub fn unexpected_boolean(value: &str) -> String {
        format!("Syntax error: unexpected {} value, please check boolean expression syntax", value)
    }
    
    pub fn unexpected_paren(paren: &str) -> String {
        format!("Syntax error: unexpected {} parenthesis, please check if parentheses match", paren)
    }
    
    pub fn unexpected_brace(brace: &str) -> String {
        format!("Syntax error: unexpected {} brace, please check if braces match", brace)
    }
    
    pub fn unexpected_comma() -> String {
        "Syntax error: unexpected comma, please check parameter or list syntax".to_string()
    }
    
    pub fn unexpected_colon() -> String {
        "Syntax error: unexpected colon, please check type declaration syntax".to_string()
    }
    
    pub fn unexpected_comparison_op(op: &str) -> String {
        format!("Syntax error: unexpected {} sign, please check comparison expression syntax", op)
    }
    
    pub fn unexpected_range() -> String {
        "Syntax error: unexpected range operator, please check range expression syntax".to_string()
    }
    
    pub fn unexpected_type(type_name: &str) -> String {
        format!("Syntax error: unexpected {} type keyword, please check type declaration syntax", type_name)
    }
    
    pub fn unexpected_token(token: &str) -> String {
        format!("Syntax error, unexpected token: {}", token)
    }
    
    // Runtime 错误
    pub fn undefined_identifier(name: &str) -> String {
        format!("Undefined identifier: \"{}\" is not defined", name)
    }
    
    pub fn function_arity_error(name: &str, expected: usize, actual: usize) -> String {
        format!("Function '{}' expects {} arguments, got {}", name, expected, actual)
    }
    
    pub fn undefined_function(name: &str) -> String {
        format!("Function '{}' is not defined", name)
    }
    
    // Array/List 错误
    pub fn array_type_error(expected_type: &str, actual_type: &str) -> String {
        format!("Type error in array assignment: cannot assign value of type '{}' to array of type '{}'", actual_type, expected_type)
    }
    
    pub fn help_array_assignment() -> String {
        "help: array elements must match the declared type".to_string()
    }
    
    pub fn example_array_assignment() -> String {
        "example: arr[0] = 42  // if arr is declared as <int>".to_string()
    }
    
    // 帮助信息
    pub fn help_var_initializer() -> String {
        "help: syntax is `var name = value`".to_string()
    }
    
    pub fn example_var_initializer() -> String {
        "example: var name = \"Alice\"".to_string()
    }
    
    pub fn help_undefined_identifier() -> String {
        "help: you must declare variables before using them".to_string()
    }
    
    pub fn example_undefined_identifier() -> String {
        "example: var name = \"Alice\"".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub file_path: String,
    pub source_line: String,
    pub suggestion: Option<String>,
    pub help: Option<String>,
    pub example: Option<String>,
}

impl CompilerError {
    pub fn new(message: String, line: usize, column: usize, file_path: String, source_line: String) -> Self {
        Self {
            message,
            line,
            column,
            file_path,
            source_line,
            suggestion: None,
            help: None,
            example: None,
        }
    }
    
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
    
    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
    
    pub fn with_example(mut self, example: String) -> Self {
        self.example = Some(example);
        self
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 主要错误消息
        writeln!(f, "{}", self.message)?;
        
        // 位置信息
        writeln!(f, "  --> {}:{}:{}", self.file_path, self.line, self.column)?;
        writeln!(f, "  |")?;
        
        // 源代码行
        writeln!(f, "{} | {}", self.line, self.source_line)?;
        write!(f, "  | ")?;
        
        // 指向错误的箭头
        for _i in 1..self.column {
            write!(f, " ")?;
        }
        
        // 计算错误长度（对于标识符，通常是标识符的长度）
        let error_length = if self.message.contains("is not defined") || 
                             self.message.contains("requires an initializer") || 
                             self.message.contains("is not defined") {
            // 从未定义变量名或标识符中提取长度
            if let Some(start) = self.message.find('"') {
                if let Some(end) = self.message[start+1..].find('"') {
                    end
                } else {
                    1
                }
            } else {
                1
            }
        } else {
            1
        };
        
        for _ in 0..error_length {
            write!(f, "^")?;
        }
        
        if let Some(suggestion) = &self.suggestion {
            writeln!(f)?;
            writeln!(f, "  = {}", suggestion)?;
        }
        
        // 添加帮助信息
        if let Some(help) = &self.help {
            writeln!(f)?;
            writeln!(f, "  = {}", help)?;
        }
        
        if let Some(example) = &self.example {
            writeln!(f, "  = {}", example)?;
        }
        
        Ok(())
    }
}

    // 便捷函数用于创建常见错误
pub fn create_undefined_identifier_error(name: &str, line: usize, column: usize, file_path: &str, source_line: &str) -> CompilerError {
    use error_messages::*;
    
    CompilerError::new(
        undefined_identifier(name),
        line,
        column,
        file_path.to_string(),
        source_line.to_string(),
    )
    .with_suggestion("did you mean to declare this variable?".to_string())
    .with_help(help_undefined_identifier())
    .with_example(example_undefined_identifier())
}