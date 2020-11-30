use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof, // End of file

    // Idenifiers + literals
    Ident(String), // add, foobar, x, y ...
    Int(String),   // 123456789

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::Eof => write!(f, "EOF"),

            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Int(int) => write!(f, "{}", int),

            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),

            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),

            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),

            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
        }
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    keyword_to_token(ident).unwrap_or_else(|| Token::Ident(ident.to_owned()))
}

fn keyword_to_token(keyword: &str) -> Option<Token> {
    match keyword {
        "fn" => Some(Token::Function),
        "let" => Some(Token::Let),
        _ => None,
    }
}
