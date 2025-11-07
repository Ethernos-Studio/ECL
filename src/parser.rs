use crate::token::Token;
use crate::ast::ASTNode;
use crate::error::CompilerError;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: String, line: usize, column: usize) -> Self {
        Self { message, line, column }
    }
}

pub struct Parser {
    lexer: crate::lexer::Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: crate::lexer::Lexer) -> Self {
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }
    
    pub fn parse(&mut self, file_path: &str, source_lines: &[String]) -> Result<Vec<ASTNode>, CompilerError> {
        let mut statements = Vec::new();
        
        while !matches!(self.current_token, Token::Eof) {
            let (line, column) = self.lexer.get_position();
            
            // Check for lexer errors first
            if let Token::Error(msg) = &self.current_token {
                let source_line = source_lines.get(line - 1).unwrap_or(&String::new()).clone();
                return Err(CompilerError::new(
                    msg.clone(),
                    line,
                    column,
                    file_path.to_string(),
                    source_line,
                ));
            }
            
            let stmt = self.parse_statement()
                .ok_or_else(|| {
                    let source_line = source_lines.get(line - 1).unwrap_or(&String::new()).clone();
                    CompilerError::new(
                        format!("Syntax error at token: {:?}", self.current_token),
                        line,
                        column,
                        file_path.to_string(),
                        source_line,
                    )
                })?;
            statements.push(stmt);
        }
        
        Ok(statements)
    }
    
    fn parse_statement(&mut self) -> Option<ASTNode> {
        // Check for lexer errors first
        if let Token::Error(_) = &self.current_token {
            return None;
        }
        
        match &self.current_token.clone() {
            Token::Var => {
                self.advance();
                if let Token::Identifier(name) = &self.current_token.clone() {
                    let var_name = name.clone();
                    self.advance();
                    
                    if !matches!(self.current_token, Token::Equal) {
                        return None;
                    }
                    self.advance();
                    
                    let expr = self.parse_expression()?;
                    
                    if matches!(self.current_token, Token::Semicolon) {
                        self.advance();
                    }
                    
                    Some(ASTNode::Var(var_name, Box::new(expr)))
                } else {
                    None
                }
            }
            Token::For => {
                self.advance();
                if let Token::Identifier(var) = &self.current_token.clone() {
                    let var_name = var.clone();
                    self.advance();
                    
                    // 检查 'in' 关键字
                    if !matches!(self.current_token, Token::In) {
                        return None;
                    }
                    self.advance();
                    
                    // 解析范围表达式
                    let range_expr = self.parse_expression()?;
                    
                    let body = if matches!(self.current_token, Token::LeftBrace) {
                        self.advance();
                        let mut statements = Vec::new();
                        while !matches!(self.current_token, Token::RightBrace) {
                            if let Some(stmt) = self.parse_statement() {
                                statements.push(stmt);
                            } else {
                                break;
                            }
                        }
                        if matches!(self.current_token, Token::RightBrace) {
                            self.advance();
                        }
                        statements
                    } else {
                        if let Some(stmt) = self.parse_statement() {
                            vec![stmt]
                        } else {
                            vec![]
                        }
                    };
                    
                    Some(ASTNode::For(var_name, Box::new(range_expr), body))
                } else {
                    None
                }
            }
            Token::If => {
                self.advance();
                let condition = self.parse_expression()?;
                
                let then_branch = if matches!(self.current_token, Token::LeftBrace) {
                    self.advance();
                    let mut statements = Vec::new();
                    while !matches!(self.current_token, Token::RightBrace) {
                        if let Some(stmt) = self.parse_statement() {
                            statements.push(stmt);
                        } else {
                            break;
                        }
                    }
                    if matches!(self.current_token, Token::RightBrace) {
                        self.advance();
                    }
                    statements
                } else {
                    if let Some(stmt) = self.parse_statement() {
                        vec![stmt]
                    } else {
                        vec![]
                    }
                };
                
                let else_branch = if matches!(self.current_token, Token::Else) {
                    self.advance();
                    if matches!(self.current_token, Token::LeftBrace) {
                        self.advance();
                        let mut statements = Vec::new();
                        while !matches!(self.current_token, Token::RightBrace) {
                            if let Some(stmt) = self.parse_statement() {
                                statements.push(stmt);
                            } else {
                                break;
                            }
                        }
                        if matches!(self.current_token, Token::RightBrace) {
                            self.advance();
                        }
                        Some(statements)
                    } else {
                        if let Some(stmt) = self.parse_statement() {
                            Some(vec![stmt])
                        } else {
                            None
                        }
                    }
                } else {
                    None
                };
                
                Some(ASTNode::If(Box::new(condition), then_branch, else_branch))
            }
            Token::Print => {
                self.advance();
                if !matches!(self.current_token, Token::LeftParen) {
                    return None;
                }
                self.advance();
                
                let expr = self.parse_expression()?;
                
                if !matches!(self.current_token, Token::RightParen) {
                    return None;
                }
                self.advance();
                
                if matches!(self.current_token, Token::Semicolon) {
                    self.advance();
                }
                
                Some(ASTNode::Print(Box::new(expr)))
            }
            Token::Println => {
                self.advance();
                if !matches!(self.current_token, Token::LeftParen) {
                    return None;
                }
                self.advance();
                
                let expr = self.parse_expression()?;
                
                if !matches!(self.current_token, Token::RightParen) {
                    return None;
                }
                self.advance();
                
                if matches!(self.current_token, Token::Semicolon) {
                    self.advance();
                }
                
                Some(ASTNode::Println(Box::new(expr)))
            }
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance();
                
                if matches!(self.current_token, Token::Equal) {
                    self.advance();
                    let expr = self.parse_expression()?;
                    
                    if matches!(self.current_token, Token::Semicolon) {
                        self.advance();
                    }
                    
                    Some(ASTNode::Assign(var_name, Box::new(expr)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    fn parse_expression(&mut self) -> Option<ASTNode> {
        self.parse_binary_expression()
    }
    
    fn parse_binary_expression(&mut self) -> Option<ASTNode> {
        let mut left = self.parse_primary()?;
        
        while let Some(op) = self.get_binary_op() {
            self.advance();
            let right = self.parse_primary()?;
            left = ASTNode::BinaryOp(Box::new(left), op, Box::new(right));
        }
        
        Some(left)
    }
    
    fn parse_primary(&mut self) -> Option<ASTNode> {
        match &self.current_token.clone() {
            Token::Number(n) => {
                let value = *n;
                self.advance();
                Some(ASTNode::Number(value))
            }
            Token::String(s) => {
                let value = s.clone();
                self.advance();
                Some(ASTNode::String(value))
            }
            Token::Identifier(name) => {
                let value = name.clone();
                self.advance();
                Some(ASTNode::Identifier(value))
            }
            Token::LeftParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                if !matches!(self.current_token, Token::RightParen) {
                    return None;
                }
                self.advance(); // consume ')'
                Some(expr)
            }
            _ => {
                // Handle undefined identifiers and syntax errors
                None
            }
        }
    }
    
    fn get_binary_op(&self) -> Option<String> {
        match &self.current_token {
            Token::Plus => Some("+".to_string()),
            Token::Minus => Some("-".to_string()),
            Token::Multiply => Some("*".to_string()),
            Token::Divide => Some("/".to_string()),
            Token::Less => Some("<".to_string()),
            Token::Greater => Some(">".to_string()),
            Token::Range => Some("..".to_string()),
            _ => None,
        }
    }
    
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}