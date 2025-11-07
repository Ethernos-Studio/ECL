use crate::token::Token;
use crate::ast::{ASTNode, Type};
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
            Token::Func => {
                self.advance();
                if let Token::Identifier(name) = &self.current_token.clone() {
                    let func_name = name.clone();
                    self.advance();
                    
                    if !matches!(self.current_token, Token::LeftParen) {
                        return None;
                    }
                    self.advance();
                    
                    // Parse parameters
                    let mut params = Vec::new();
                    while !matches!(self.current_token, Token::RightParen) {
                        if let Token::Identifier(param) = &self.current_token.clone() {
                            params.push(param.clone());
                            self.advance();
                            
                            if matches!(self.current_token, Token::Comma) {
                                self.advance();
                            }
                        } else {
                            return None;
                        }
                    }
                    
                    if !matches!(self.current_token, Token::RightParen) {
                        return None;
                    }
                    self.advance();
                    
                    // Parse function body
                    if !matches!(self.current_token, Token::LeftBrace) {
                        return None;
                    }
                    self.advance();
                    
                    let mut body = Vec::new();
                    while !matches!(self.current_token, Token::RightBrace) {
                        if let Some(stmt) = self.parse_statement() {
                            body.push(stmt);
                        } else {
                            break;
                        }
                    }
                    
                    if !matches!(self.current_token, Token::RightBrace) {
                        return None;
                    }
                    self.advance();
                    
                    Some(ASTNode::Function(func_name, params, body))
                } else {
                    None
                }
            }
            Token::Expr => {
                self.advance();
                if let Token::Identifier(name) = &self.current_token.clone() {
                    let func_name = name.clone();
                    self.advance();
                    
                    if !matches!(self.current_token, Token::LeftParen) {
                        return None;
                    }
                    self.advance();
                    
                    // Parse parameters with types: l a, r b, etc.
                    let mut params = Vec::new();
                    while !matches!(self.current_token, Token::RightParen) {
                        // Parse parameter type (l, r, l1, r1, argv, etc.)
                        if let Token::Identifier(param_type) = &self.current_token.clone() {
                            let ptype = param_type.clone();
                            self.advance();
                            
                            // Parse parameter name
                            if let Token::Identifier(param_name) = &self.current_token.clone() {
                                let pname = param_name.clone();
                                self.advance();
                                
                                params.push((ptype, pname));
                                
                                if matches!(self.current_token, Token::Comma) {
                                    self.advance();
                                }
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    }
                    
                    if !matches!(self.current_token, Token::RightParen) {
                        return None;
                    }
                    self.advance();
                    
                    // Parse function body
                    if !matches!(self.current_token, Token::LeftBrace) {
                        return None;
                    }
                    self.advance();
                    
                    let mut body = Vec::new();
                    while !matches!(self.current_token, Token::RightBrace) {
                        if let Some(stmt) = self.parse_statement() {
                            body.push(stmt);
                        } else {
                            break;
                        }
                    }
                    
                    if !matches!(self.current_token, Token::RightBrace) {
                        return None;
                    }
                    self.advance();
                    
                    Some(ASTNode::Expr(func_name, params, body))
                } else {
                    None
                }
            }
            Token::Return => {
                self.advance();
                let expr = self.parse_expression()?;
                
                if matches!(self.current_token, Token::Semicolon) {
                    self.advance();
                }
                
                Some(ASTNode::Return(Box::new(expr)))
            }
            Token::Var => {
                self.advance();
                
                // Check for type annotation: var <type>name = value
                let var_type = if matches!(self.current_token, Token::LessThan) {
                    self.advance(); // consume '<'
                    
                    // Parse type name
                    let type_token = self.current_token.clone();
                    self.advance(); // consume type name
                    
                    if !matches!(self.current_token, Token::GreaterThan) {
                        return None;
                    }
                    self.advance(); // consume '>'
                    
                    match type_token {
                        Token::Int => Some(Type::Int),
                        Token::Str => Some(Type::Str),
                        Token::Bool => Some(Type::Bool),
                        Token::Float => Some(Type::Float),
                        Token::Double => Some(Type::Double),
                        _ => return None,
                    }
                } else {
                    None
                };
                
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
                    
                    if let Some(t) = var_type {
                        Some(ASTNode::TypedVar(var_name, t, Box::new(expr)))
                    } else {
                        Some(ASTNode::Var(var_name, Box::new(expr)))
                    }
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
            Token::LessThan => {
                // Handle direct type declaration: <type>name = value
                self.advance(); // consume '<'
                
                // Parse type name
                let type_token = self.current_token.clone();
                self.advance(); // consume type name
                
                if !matches!(self.current_token, Token::GreaterThan) {
                    return None;
                }
                self.advance(); // consume '>'
                
                let var_type = match type_token {
                    Token::Int => Type::Int,
                    Token::Str => Type::Str,
                    Token::Bool => Type::Bool,
                    Token::Float => Type::Float,
                    Token::Double => Type::Double,
                    _ => return None,
                };
                
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
                    
                    Some(ASTNode::TypedVar(var_name, var_type, Box::new(expr)))
                } else {
                    None
                }
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
                    // Check for function call: func_name(arg1, arg2)
                    if matches!(self.current_token, Token::LeftParen) {
                        self.advance(); // consume '('
                        
                        let mut args = Vec::new();
                        while !matches!(self.current_token, Token::RightParen) {
                            let arg = self.parse_expression()?;
                            args.push(arg);
                            
                            if matches!(self.current_token, Token::Comma) {
                                self.advance();
                            }
                        }
                        
                        if !matches!(self.current_token, Token::RightParen) {
                            return None;
                        }
                        self.advance(); // consume ')'
                        
                        Some(ASTNode::FunctionCall(var_name, args))
                    } else {
                        // Simple identifier
                        Some(ASTNode::Identifier(var_name))
                    }
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
        
        // Check for expr function call syntax: arg1 func_name arg2
        // This should be detected before regular binary operations
        match &left {
            ASTNode::Number(arg1_val) => {
                // Pattern: Number Identifier (e.g., "16 x2")
                if let Token::Identifier(func_name) = &self.current_token.clone() {
                    let fname = func_name.clone();
                    self.advance(); // consume function name
                    
                    let mut args = vec![ASTNode::Number(*arg1_val)]; // First argument
                    
                    // Parse additional arguments
                    while matches!(&self.current_token, Token::Number(_) | Token::Identifier(_) | Token::LeftParen) {
                        if let Some(arg) = self.parse_primary() {
                            args.push(arg);
                        } else {
                            break;
                        }
                    }
                    
                    return Some(ASTNode::FunctionCall(fname, args));
                }
            }
            ASTNode::Identifier(arg1_name) => {
                // Pattern: Identifier Identifier (e.g., "negate x2") or Identifier Number (e.g., "negate 16")
                if let Token::Identifier(func_name) = &self.current_token.clone() {
                    let fname = func_name.clone();
                    self.advance(); // consume function name
                    
                    let mut args = vec![ASTNode::Identifier(arg1_name.clone())]; // First argument
                    
                    // Parse additional arguments
                    while matches!(&self.current_token, Token::Number(_) | Token::Identifier(_) | Token::LeftParen) {
                        if let Some(arg) = self.parse_primary() {
                            args.push(arg);
                        } else {
                            break;
                        }
                    }
                    
                    return Some(ASTNode::FunctionCall(fname, args));
                } else if let Token::Number(_) = &self.current_token {
                    // Pattern: Identifier Number (e.g., "negate 16")
                    // This might be an expr function call where the first argument is an identifier
                    // and the function name is actually the identifier we just parsed
                    let mut args = Vec::new();
                    
                    // Parse arguments (starting with the number after the function name)
                    while matches!(&self.current_token, Token::Number(_) | Token::Identifier(_) | Token::LeftParen) {
                        if let Some(arg) = self.parse_primary() {
                            args.push(arg);
                        } else {
                            break;
                        }
                    }
                    
                    // If we have arguments, this is likely an expr call with the function name being the first identifier
                    if !args.is_empty() {
                        return Some(ASTNode::FunctionCall(arg1_name.clone(), args));
                    }
                }
            }
            _ => {}
        }
        
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
            Token::True => {
                self.advance();
                Some(ASTNode::Bool(true))
            }
            Token::False => {
                self.advance();
                Some(ASTNode::Bool(false))
            }
            Token::Minus => {
                // Handle unary minus (negative numbers)
                self.advance();
                let expr = self.parse_primary()?;
                Some(ASTNode::BinaryOp(Box::new(ASTNode::Number(0.0)), "-".to_string(), Box::new(expr)))
            }
            Token::Identifier(name) => {
                let func_name = name.clone();
                self.advance();
                
                // Check if this is a function call
                if matches!(self.current_token, Token::LeftParen) {
                    self.advance(); // consume '('
                    
                    let mut args = Vec::new();
                    while !matches!(self.current_token, Token::RightParen) {
                        let arg = self.parse_expression()?;
                        args.push(arg);
                        
                        if matches!(self.current_token, Token::Comma) {
                            self.advance();
                        }
                    }
                    
                    if !matches!(self.current_token, Token::RightParen) {
                        return None;
                    }
                    self.advance(); // consume ')'
                    
                    Some(ASTNode::FunctionCall(func_name, args))
                } else {
                    Some(ASTNode::Identifier(func_name))
                }
            }
            Token::LeftParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                if !matches!(self.current_token, Token::RightParen) {
                    return None;
                }
                self.advance(); // consume ')'
                
                // Check for expr function call syntax: (expr)func_name
                if let Token::Identifier(func_name) = &self.current_token.clone() {
                    let fname = func_name.clone();
                    self.advance();
                    
                    // Check if this is followed by another expression (for syntax like (2)multiply(8))
                    if matches!(self.current_token, Token::LeftParen) {
                        self.advance(); // consume '('
                        
                        let mut args = vec![expr]; // First argument is the expression in parentheses
                        
                        // Parse additional arguments
                        while !matches!(self.current_token, Token::RightParen) {
                            let arg = self.parse_expression()?;
                            args.push(arg);
                            
                            if matches!(self.current_token, Token::Comma) {
                                self.advance();
                            }
                        }
                        
                        if !matches!(self.current_token, Token::RightParen) {
                            return None;
                        }
                        self.advance(); // consume ')'
                        
                        Some(ASTNode::FunctionCall(fname, args))
                    } else {
                        // Simple expr call: (expr)func_name
                        Some(ASTNode::FunctionCall(fname, vec![expr]))
                    }
                } else {
                    Some(expr)
                }
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