#[derive(Debug, Clone)]
pub enum Token {
    Print,
    Println,
    Var,
    For,
    In,
    If,
    Else,
    While,
    Input,
    Func,
    Expr,
    Return,
    True,
    False,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,    // [
    RightBracket,   // ]
    Semicolon,
    Comma,
    Colon,
    Equal,
    EqualEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
    Less,           // 已使用，用于比较操作
    Greater,        // 已使用，用于比较操作
    LessEqual,      // <=
    GreaterEqual,   // >=
    Range,  // .. 运算符
    LessThan,       // 已使用，用于类型声明 <type>name
    GreaterThan,    // 已使用，用于类型声明 <type>name
    // 类型关键字
    Int,
    Str,
    Bool,
    Float,
    Double,
    Identifier(String),
    Number(f64),
    String(String),
    Error(String),
    Eof,
}