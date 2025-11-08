use crate::token::Token;
use crate::ast::{ASTNode, Type};
use crate::error::{CompilerError, error_messages};

pub struct Parser {
    lexer: crate::lexer::Lexer,
    current_token: Token,
    error_context: Option<String>,
}

impl Parser {
    pub fn new(mut lexer: crate::lexer::Lexer) -> Self {
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            error_context: None,
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
            
            // 特殊处理变量声明错误，提供更友好的错误信息
            if matches!(self.current_token, Token::Var) {
                let var_line = line;
                let var_column = column;
                let source_line = source_lines.get(var_line - 1).unwrap_or(&String::new()).clone();
                
                // 检查是否是 var name; 模式（缺少初始化器）
                let line_content = source_line.trim();
                if let Some(var_pos) = line_content.find("var") {
                    let after_var = &line_content[var_pos + 3..].trim();
                    // 查找第一个标识符（变量名）
                    if let Some(space_pos) = after_var.find(|c: char| !c.is_alphanumeric() && c != '_') {
                        let var_name = &after_var[..space_pos];
                        let after_name = after_var[space_pos..].trim();
                        // 如果后面直接是分号或结束，说明缺少初始化器
                        if after_name.starts_with(';') || after_name.is_empty() {
                            // 变量声明缺少初始化器
                            return Err(CompilerError::new(
                                error_messages::unexpected_var(),
                                var_line,
                                var_column + 3 + var_name.len(), // var + 空格 + 变量名后的位置
                                file_path.to_string(),
                                source_line,
                            ).with_help(error_messages::help_var_initializer())
                             .with_example(error_messages::example_var_initializer()));
                        }
                    }
                }
            }
            
            let stmt = self.parse_statement()
                .ok_or_else(|| {
                    let source_line = source_lines.get(line - 1).unwrap_or(&String::new()).clone();
                    let error_msg = match &self.current_token {
                        Token::Semicolon => error_messages::unexpected_semicolon(),
                        Token::Eof =>       error_messages::unexpected_eof(),
                        Token::Plus =>      error_messages::unexpected_operator("plus"),
                        Token::Minus =>     error_messages::unexpected_operator("minus"),
                        Token::Multiply =>  error_messages::unexpected_operator("multiplication"),
                        Token::Divide =>    error_messages::unexpected_operator("division"),
                        Token::Equal =>     error_messages::unexpected_operator("equals"),
                        Token::Print =>     error_messages::unexpected_keyword("print"),
                        Token::Println =>   error_messages::unexpected_keyword("println"),
                        Token::Var =>       error_messages::unexpected_var(),
                        Token::For =>       error_messages::unexpected_loop_keyword("for"),
                        Token::In =>        error_messages::unexpected_loop_keyword("in"),
                        Token::If =>        error_messages::unexpected_conditional_keyword("if"),
                        Token::Else =>      error_messages::unexpected_else(),
                        Token::While =>     error_messages::unexpected_loop_keyword("while"),
                        Token::Input =>     error_messages::unexpected_input(),
                        Token::Func =>      error_messages::unexpected_func(),
                        Token::Expr =>      error_messages::unexpected_expr(),
                        Token::Return =>    error_messages::unexpected_return(),
                        Token::True =>      error_messages::unexpected_boolean("true"),
                        Token::False =>     error_messages::unexpected_boolean("false"),
                        Token::LeftParen => error_messages::unexpected_paren("left"),
                        Token::RightParen =>error_messages::unexpected_paren("right"),
                        Token::LeftBrace => error_messages::unexpected_brace("left"),
                        Token::RightBrace =>error_messages::unexpected_brace("right"),
                        Token::Comma =>     error_messages::unexpected_comma(),
                        Token::Colon =>     error_messages::unexpected_colon(),
                        Token::Less =>      error_messages::unexpected_comparison_op("less than"),
                        Token::Greater =>   error_messages::unexpected_comparison_op("greater than"),
                        Token::LessEqual => error_messages::unexpected_comparison_op("less than or equal"),
                        Token::GreaterEqual => error_messages::unexpected_comparison_op("greater than or equal"),
                        Token::Range =>     error_messages::unexpected_range(),
                        Token::LessThan =>  error_messages::unexpected_comparison_op("less than"),
                        Token::GreaterThan => error_messages::unexpected_comparison_op("greater than"),
                        Token::Int =>       error_messages::unexpected_type("int"),
                        Token::Str =>       error_messages::unexpected_type("str"),
                        Token::Bool =>      error_messages::unexpected_type("bool"),
                        Token::Float =>     error_messages::unexpected_type("float"),
                        Token::Double =>    error_messages::unexpected_type("double"),
                        token => error_messages::unexpected_token(&format!("{:?}", token)),
                    };
                    
                    // Append error context if available
                    let full_error_msg = if let Some(context) = &self.error_context {
                        format!("{}\nHere are some possible causes and solutions for the syntax error:\n{}", error_msg, context)
                    } else {
                        error_msg
                    };
                    
                    // Clear the error context
                    self.error_context = None;
                    
                    CompilerError::new(
                        full_error_msg,
                        line,
                        column,
                        file_path.to_string(),
                        source_line,
                    )
                })?;
            statements.push(stmt);
            
            // Consume optional semicolon after statement
            if matches!(self.current_token, Token::Semicolon) {
                self.advance();
            }
        }
        
        Ok(statements)
    }
    
    fn parse_statement(&mut self) -> Option<ASTNode> {
        // Clear any previous error context
        self.error_context = None;
        
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
                    
                    // Check if this is an array declaration: name[size]
                    if matches!(self.current_token, Token::LeftBracket) {
                        // This is an array declaration: var <type>name[size] = {init_values}
                        if let None = var_type {
                            return None; // Array must have a type
                        }
                        
                        self.advance(); // consume '['
                        
                        // Parse the size expression
                        let size_expr = self.parse_expression()?;
                        
                        if !matches!(self.current_token, Token::RightBracket) {
                            return None;
                        }
                        self.advance(); // consume ']'
                        
                        if !matches!(self.current_token, Token::Equal) {
                            return None;
                        }
                        self.advance(); // consume '='
                        
                        // Parse initialization expression - should be a list of values
                        let init_expr = self.parse_expression()?;
                        
                        if matches!(self.current_token, Token::Semicolon) {
                            self.advance();
                        }
                        
                        // Extract the size value from the expression
                        let size_val = match size_expr {
                            ASTNode::Number(n) => n as usize,
                            _ => 0, // Default to 0 if not a number (error case)
                        };
                        
                        // For array initialization, we look for the array_init function call
                        let init_values = match init_expr {
                            ASTNode::FunctionCall(ref func_name, ref args) if func_name == "array_init" => {
                                args.clone()  // Get the values from the array initialization
                            }
                            _ => vec![init_expr], // If it's a single value, use it to initialize all elements
                        };
                        
                        if let Some(t) = var_type {
                            Some(ASTNode::ArrayDecl(var_name, t, size_val, init_values))
                        } else {
                            None // Should not happen since we checked earlier
                        }
                    } else {
                        if !matches!(self.current_token, Token::Equal) {
                            return None;
                        }
                        self.advance();
                        
                        let expr = self.parse_expression()?;
                        
                        // Check if this is a list initialization: var name = []
                        let result = match &expr {
                            ASTNode::FunctionCall(func_name, args) if func_name == "list_init" && args.is_empty() => {
                                // This is a list initialization
                                ASTNode::ListDecl(var_name, vec![])
                            }
                            ASTNode::FunctionCall(func_name, args) if func_name == "array_init" => {
                                // This is a list initialization with values
                                ASTNode::ListDecl(var_name, args.clone())
                            }
                            _ => {
                                // Regular variable
                                if let Some(t) = var_type {
                                    ASTNode::TypedVar(var_name, t, Box::new(expr))
                                } else {
                                    ASTNode::Var(var_name, Box::new(expr))
                                }
                            }
                        };
                        
                        if matches!(self.current_token, Token::Semicolon) {
                            self.advance();
                        }
                        
                        Some(result)
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
            Token::While => {
                self.advance();
                
                if !matches!(self.current_token, Token::LeftParen) {
                    return None;
                }
                self.advance();
                
                let condition = self.parse_expression()?;
                
                if !matches!(self.current_token, Token::RightParen) {
                    return None;
                }
                self.advance();
                
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
                
                Some(ASTNode::While(Box::new(condition), body))
            }
            Token::Input => {
                self.advance();
                if !matches!(self.current_token, Token::LeftParen) {
                    return None;
                }
                self.advance();
                
                let prompt = self.parse_expression()?;
                
                if !matches!(self.current_token, Token::Comma) {
                    return None;
                }
                self.advance();
                
                if let Token::Identifier(var_name) = &self.current_token.clone() {
                    let var = var_name.clone();
                    self.advance();
                    
                    if !matches!(self.current_token, Token::RightParen) {
                        return None;
                    }
                    self.advance();
                    
                    if matches!(self.current_token, Token::Semicolon) {
                        self.advance();
                    }
                    
                    Some(ASTNode::Input(Box::new(prompt), var))
                } else {
                    None
                }
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
                
                // Check for index access: name[index]
                if matches!(self.current_token, Token::LeftBracket) {
                    // This is an array/list index access: name[index]
                    let identifier_node = {
                        let (line, column) = self.lexer.get_position();
                        let pos = crate::ast::Position::new(line, column);
                        ASTNode::Identifier(var_name.clone(), pos)
                    };
                    
                    self.advance(); // consume '['
                    
                    let index_expr = self.parse_expression()?;
                    
                    if !matches!(self.current_token, Token::RightBracket) {
                        return None;
                    }
                    self.advance(); // consume ']'
                    
                    // Check if this is an assignment: name[index] = value
                        if matches!(self.current_token, Token::Equal) {
                            self.advance(); // consume '='
                            // Get position right after the '=' to mark the assignment location
                            let (line, column) = self.lexer.get_position();
                            let pos = crate::ast::Position::new(line, column);
                            let value_expr = self.parse_expression()?;
                            
                            if matches!(self.current_token, Token::Semicolon) {
                                self.advance();
                            }
                            
                            Some(ASTNode::IndexAssign(Box::new(identifier_node), Box::new(index_expr), Box::new(value_expr), pos))
                        } else {
                            // Just index access
                            Some(ASTNode::IndexAccess(Box::new(identifier_node), Box::new(index_expr)))
                        }
                    } else if matches!(self.current_token, Token::Equal) {
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
                        let (line, column) = self.lexer.get_position();
                        let pos = crate::ast::Position::new(line, column);
                        Some(ASTNode::Identifier(var_name, pos))
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
            ASTNode::Identifier(arg1_name, _) => {
                // Pattern: Identifier Identifier (e.g., "negate x2") or Identifier Number (e.g., "negate 16")
                if let Token::Identifier(func_name) = &self.current_token.clone() {
                    let fname = func_name.clone();
                    self.advance(); // consume function name
                    
                    let (line, column) = self.lexer.get_position();
                    let pos = crate::ast::Position::new(line, column);
                    let mut args = vec![ASTNode::Identifier(arg1_name.clone(), pos)]; // First argument
                    
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
            let right = self.parse_expression()?;
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
            Token::If => {
                // Parse if expression: if (condition) then_expr else else_expr
                self.advance(); // consume 'if'
                
                if !matches!(self.current_token, Token::LeftParen) {
                    return None;
                }
                self.advance(); // consume '('
                
                let condition = self.parse_expression()?;
                
                if !matches!(self.current_token, Token::RightParen) {
                    return None;
                }
                self.advance(); // consume ')'
                
                let then_expr = self.parse_expression()?;
                
                // Check for 'else'
                if !matches!(self.current_token, Token::Else) {
                    return None;
                }
                self.advance(); // consume 'else'
                
                let else_expr = self.parse_expression()?;
                
                Some(ASTNode::IfExpr(Box::new(condition), Box::new(then_expr), Box::new(else_expr)))
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
                } else if matches!(self.current_token, Token::LeftBracket) {
                    // This is an array/list index access: name[index]
                    let identifier_node = {
                        let (line, column) = self.lexer.get_position();
                        let pos = crate::ast::Position::new(line, column);
                        ASTNode::Identifier(func_name, pos)
                    };
                    
                    self.advance(); // consume '['
                    
                    let index_expr = self.parse_expression()?;
                    
                    if !matches!(self.current_token, Token::RightBracket) {
                        return None;
                    }
                    self.advance(); // consume ']'
                    
                    Some(ASTNode::IndexAccess(Box::new(identifier_node), Box::new(index_expr)))
                } else {
                    let (line, column) = self.lexer.get_position();
                    let pos = crate::ast::Position::new(line, column);
                    Some(ASTNode::Identifier(func_name, pos))
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
            Token::LeftBrace => {
                // Parse array/list initialization: {value1, value2, ...}
                self.advance(); // consume '{'
                
                let mut elements = Vec::new();
                while !matches!(self.current_token, Token::RightBrace) {
                    let element = self.parse_expression()?;
                    elements.push(element);
                    
                    if matches!(self.current_token, Token::Comma) {
                        self.advance(); // consume ','
                    } else if matches!(self.current_token, Token::RightBrace) {
                        break;
                    } else {
                        // If there's no comma and not the closing brace, it's an error
                        return None;
                    }
                }
                
                if !matches!(self.current_token, Token::RightBrace) {
                    return None;
                }
                self.advance(); // consume '}'
                
                // Return a special node that represents an array/list initialization
                // We'll handle the distinction in the context where it's used
                Some(ASTNode::FunctionCall("array_init".to_string(), elements))
            }
            Token::LeftBracket => {
                // Parse an empty list: []
                self.advance(); // consume '['
                
                if !matches!(self.current_token, Token::RightBracket) {
                    return None;
                }
                self.advance(); // consume ']'
                
                // Return an empty list
                Some(ASTNode::FunctionCall("list_init".to_string(), vec![]))
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
            Token::LessEqual => Some("<=".to_string()),
            Token::GreaterEqual => Some(">=".to_string()),
            Token::LessThan => Some("<".to_string()),
            Token::GreaterThan => Some(">".to_string()),
            Token::EqualEqual => Some("==".to_string()),
            Token::Range => Some("..".to_string()),
            _ => None,
        }
    }
    
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}