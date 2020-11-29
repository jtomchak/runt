use crate::token;
use crate::token::Token;
use std::iter::Peekable;
use std::mem;
use std::str::Chars;

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn next_token() {
        let input = r#"=+(){},;"#;

        let tests = [
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input.to_owned());

        for (i, expected_token) in tests.iter().enumerate() {
            let token = lexer.next_token();

            assert_eq!(&token, expected_token, "tests[{}] - token", i);
        }
    }
}
