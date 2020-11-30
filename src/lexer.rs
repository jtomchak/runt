use crate::token;
use crate::token::Token;
use std::iter::Peekable;
use std::mem;
use std::str::Chars;

pub struct Lexer {
    input: String,
    // Current position of input
    position: usize,
    // current reading position of input (after current char)
    read_position: usize,
    // current character under examination
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\u{0}',
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        match self.ch {
            '=' => {
                tok = Token::Assign;
            }
            ';' => {
                tok = Token::Semicolon;
            }
            '(' => tok = Token::Lparen,
            ')' => tok = Token::Rparen,
            '{' => tok = Token::Lbrace,
            '}' => tok = Token::Rbrace,
            '+' => tok = Token::Plus,
            ',' => tok = Token::Comma,
            '\u{0}' => tok = Token::Eof,
            _ => tok = Token::Illegal,
        }

        self.read_char();
        tok
    }
    /**
     * Get the next char and advance our position on the input string
     * The position gets updated from read_position
     * Then read_position is incremented by 1
     */
    fn read_char(self: &mut Self) {
        self.ch = self
            .input
            .chars()
            .nth(self.read_position)
            .unwrap_or('\u{0}');
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn next_token() {
        let input = r#"=+(){},;"#;

        let tests = [
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input.to_owned());

        for (i, expected_token) in tests.iter().enumerate() {
            let token = lexer.next_token();

            assert_eq!(&token, expected_token, "tests[{}] - token", i);
        }
    }
}
