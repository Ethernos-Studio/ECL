use std::fmt;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub file_path: String,
    pub source_line: String,
    pub suggestion: Option<String>,
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
        }
    }
    
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
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
        let error_length = if self.message.contains("is not defined") {
            // 从未定义变量名中提取长度
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
            write!(f, " {}", suggestion)?;
        }
        
        Ok(())
    }
}

pub fn format_undefined_identifier_error(name: &str, line: usize, column: usize, file_path: &str, source_line: &str) -> CompilerError {
    CompilerError::new(
        format!("UndefinedIdentifier: \"{}\" is not defined", name),
        line,
        column,
        file_path.to_string(),
        source_line.to_string(),
    ).with_suggestion("did you mean to declare this variable?".to_string())
}