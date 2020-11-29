use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF, // End of file

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
