use crate::token::Token;

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
                self.position += 1;
                self.column += 1;
                Token::Equal
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
                    Token::Error("Unexpected character: '.'. Did you mean '..' for range?".to_string())
                }
            }
            '<' => {
                self.position += 1;
                self.column += 1;
                Token::LessThan
            }
            '>' => {
                self.position += 1;
                self.column += 1;
                Token::GreaterThan
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
                Token::Error(format!("Unexpected character: '{}' at line {}, column {}", error_char, self.line, self.column - 1))
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
            if self.input[self.position] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            result.push(self.input[self.position]);
            self.position += 1;
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