use crate::token::Token;
use crate::error::error_messages;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn get_position(&self) -> (usize, usize) {
        (self.line, self.column)
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Token::Eof;
        }
        
        let ch = self.input[self.position];
        
        match ch {
            '(' => {
                self.position += 1;
                self.column += 1;
                Token::LeftParen
            }
            ')' => {
                self.position += 1;
                self.column += 1;
                Token::RightParen
            }
            '{' => {
                self.position += 1;
                self.column += 1;
                Token::LeftBrace
            }
            '}' => {
                self.position += 1;
                self.column += 1;
                Token::RightBrace
            }
            ';' => {
                self.position += 1;
                self.column += 1;
                Token::Semicolon
            }
            ',' => {
                self.position += 1;
                self.column += 1;
                Token::Comma
            }
            ':' => {
                self.position += 1;
                self.column += 1;
                Token::Colon
            }
            '=' => {
                // Check for '=='
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '=' {
                    self.position += 2;
                    self.column += 2;
                    Token::EqualEqual
                } else {
                    self.position += 1;
                    self.column += 1;
                    Token::Equal
                }
            }
            '+' => {
                self.position += 1;
                self.column += 1;
                Token::Plus
            }
            '-' => {
                self.position += 1;
                self.column += 1;
                Token::Minus
            }
            '*' => {
                self.position += 1;
                self.column += 1;
                Token::Multiply
            }
            '/' => {
                self.position += 1;
                self.column += 1;
                Token::Divide
            }
            '.' => {
                // 检查是否是 .. 运算符
                if self.position + 1 < self.input.len() && self.input[self.position + 1] == '.' {
                    self.position += 2;
                    self.column += 2;
                    Token::Range
                } else {
                    self.position += 1;
                    self.column += 1;
                    Token::Error(error_messages::unexpected_dot())
                }
            }
            '<' => {
                self.position += 1;
                self.column += 1;
                // Check for <=
                if self.position < self.input.len() && self.input[self.position] == '=' {
                    self.position += 1;
                    self.column += 1;
                    Token::LessEqual
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                self.position += 1;
                self.column += 1;
                // Check for >=
                if self.position < self.input.len() && self.input[self.position] == '=' {
                    self.position += 1;
                    self.column += 1;
                    Token::GreaterEqual
                } else {
                    Token::GreaterThan
                }
            }
            '[' => {
                self.position += 1;
                self.column += 1;
                Token::LeftBracket
            }
            ']' => {
                self.position += 1;
                self.column += 1;
                Token::RightBracket
            }
            '"' => {
                self.position += 1;
                self.read_string()
            }
            _ if ch.is_alphabetic() => {
                self.read_identifier()
            }
            _ if ch.is_numeric() => {
                self.read_number()
            }
            _ => {
                let error_char = ch;
                self.position += 1;
                self.column += 1;
                Token::Error(error_messages::unexpected_character(error_char, self.line, self.column - 1))
            }
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            if self.input[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
                self.position += 1;
            } else if self.input[self.position].is_whitespace() {
                self.column += 1;
                self.position += 1;
            } else if self.input[self.position] == '/' && self.position + 1 < self.input.len() && self.input[self.position + 1] == '/' {
                // Skip single-line comment
                while self.position < self.input.len() && self.input[self.position] != '\n' {
                    self.position += 1;
                }
            } else {
                break;
            }
        }
    }
    
    fn read_string(&mut self) -> Token {
        let mut result = String::new();
        
        while self.position < self.input.len() && self.input[self.position] != '"' {
            if self.input[self.position] == '\\' {
                // Handle escape sequences
                if self.position + 1 < self.input.len() {
                    self.position += 1; // Move past the backslash
                    self.column += 1;
                    
                    match self.input[self.position] {
                        'n' => {
                            result.push('\n');
                            self.position += 1;
                            self.column += 1;
                        }
                        't' => {
                            result.push('\t');
                            self.position += 1;
                            self.column += 1;
                        }
                        '"' => {
                            result.push('"');
                            self.position += 1;
                            self.column += 1;
                        }
                        '\\' => {
                            result.push('\\');
                            self.position += 1;
                            self.column += 1;
                        }
                        '\'' => {
                            result.push('\'');
                            self.position += 1;
                            self.column += 1;
                        }
                        'u' => {
                            // Handle Unicode escape sequences: \u{XXXX} or \uXXXX
                            self.position += 1;
                            self.column += 1;
                            
                            if self.position < self.input.len() && self.input[self.position] == '{' {
                                // \u{XXXX} format
                                self.position += 1;
                                self.column += 1;
                                
                                let mut hex_digits = String::new();
                                while self.position < self.input.len() && self.input[self.position] != '}' {
                                    hex_digits.push(self.input[self.position]);
                                    self.position += 1;
                                    self.column += 1;
                                }
                                
                                if self.position < self.input.len() && self.input[self.position] == '}' {
                                    self.position += 1;
                                    self.column += 1;
                                    
                                    match u32::from_str_radix(&hex_digits, 16) {
                                        Ok(code_point) => {
                                            match char::from_u32(code_point) {
                                                Some(ch) => result.push(ch),
                                                None => {
                                                    // Invalid Unicode code point
                                                    result.push_str(&format!("\\u{{{}}}", hex_digits));
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            // Invalid hex digits
                                            result.push_str(&format!("\\u{{{}}}", hex_digits));
                                        }
                                    }
                                } else {
                                    // Missing closing brace
                                    result.push_str("\\u{");
                                    result.push_str(&hex_digits);
                                }
                            } else if self.position + 3 < self.input.len() {
                                // \uXXXX format (4 hex digits)
                                let hex_digits: String = self.input[self.position..self.position + 4].iter().collect();
                                
                                match u32::from_str_radix(&hex_digits, 16) {
                                    Ok(code_point) => {
                                        match char::from_u32(code_point) {
                                            Some(ch) => {
                                                result.push(ch);
                                                self.position += 4;
                                                self.column += 4;
                                            }
                                            None => {
                                                // Invalid Unicode code point
                                                result.push_str("\\u");
                                                result.push_str(&hex_digits);
                                                self.position += 4;
                                                self.column += 4;
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        // Invalid hex digits
                                        result.push_str("\\u");
                                        result.push_str(&hex_digits);
                                        self.position += 4;
                                        self.column += 4;
                                    }
                                }
                            } else {
                                // Not enough characters for \uXXXX
                                result.push_str("\\u");
                            }
                        }
                        _ => {
                            // Unknown escape sequence, treat as literal
                            result.push('\\');
                            result.push(self.input[self.position]);
                            self.position += 1;
                            self.column += 1;
                        }
                    }
                } else {
                    // Backslash at end of input, treat as literal
                    result.push('\\');
                    self.position += 1;
                    self.column += 1;
                }
            } else if self.input[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
                result.push(self.input[self.position]);
                self.position += 1;
            } else {
                self.column += 1;
                result.push(self.input[self.position]);
                self.position += 1;
            }
        }
        
        if self.position < self.input.len() && self.input[self.position] == '"' {
            self.position += 1;
            self.column += 1;
        }
        
        Token::String(result)
    }
    
    fn read_identifier(&mut self) -> Token {
        let mut result = String::new();
        
        // 第一个字符必须是字母或下划线
        if self.position < self.input.len() && (self.input[self.position].is_alphabetic() || self.input[self.position] == '_') {
            result.push(self.input[self.position]);
            self.position += 1;
            self.column += 1;
            
            // 后续字符可以是字母、数字或下划线
            while self.position < self.input.len() && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_') {
                result.push(self.input[self.position]);
                self.position += 1;
                self.column += 1;
            }
        }
        
        match result.as_str() {
            "print" => Token::Print,
            "println" => Token::Println,
            "var" => Token::Var,
            "for" => Token::For,
            "in" => Token::In,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "input" => Token::Input,
            "func" => Token::Func,
            "expr" => Token::Expr,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "int" => Token::Int,
            "str" => Token::Str,
            "bool" => Token::Bool,
            "float" => Token::Float,
            "double" => Token::Double,
            "import" => Token::Import,
            _ => Token::Identifier(result),
        }
    }
    
    fn read_number(&mut self) -> Token {
        let mut result = String::new();
        
        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            result.push(self.input[self.position]);
            self.position += 1;
            self.column += 1;
        }
        
        // 检查是否是范围运算符 .. 的一部分
        if self.position < self.input.len() && self.input[self.position] == '.' {
            // 检查下一个字符是否也是 '.'，如果是，则不消耗这个 '.'
            if self.position + 1 < self.input.len() && self.input[self.position + 1] == '.' {
                // 这是范围运算符的开始，不消耗这个 '.'
                return Token::Number(result.parse().unwrap_or(0.0));
            }
            
            // 否则，这是小数点
            result.push('.');
            self.position += 1;
            self.column += 1;
            
            while self.position < self.input.len() && self.input[self.position].is_numeric() {
                result.push(self.input[self.position]);
                self.position += 1;
                self.column += 1;
            }
        }
        
        Token::Number(result.parse().unwrap_or(0.0))
    }
}