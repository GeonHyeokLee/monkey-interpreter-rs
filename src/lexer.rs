use crate::token::{Token, TokenType};

#[test]
fn test_next_token() {
    const INPUT: &str = "let five = 5;";

    let mut tests = Vec::new();
    tests.push((TokenType::Let, "let"));
    tests.push((TokenType::Ident, "five"));
    tests.push((TokenType::Assign, "="));
    tests.push((TokenType::Int, "5"));
    tests.push((TokenType::Semicolon, ";"));
    tests.push((TokenType::Eof, ""));

    let mut l = Lexer::new(INPUT);

    for (i, tt) in tests.iter().enumerate() {
        let tok = l.next_token();
        assert_eq!(
            tok.token_type, tt.0,
            "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
            i, tt.0, tok.token_type
        );
        assert_eq!(
            tok.literal, tt.1,
            "tests[{}] - literal wrong. expected={:?}, got={:?}",
            i, tt.1, tok.literal
        );
    }
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.to_string(),
            position: usize::MIN,
            read_position: usize::MIN,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token::new(TokenType::Eq, &literal);
                } else {
                    tok = Token::new(TokenType::Assign, &self.ch.to_string());
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token::new(TokenType::NotEq, &literal);
                } else {
                    tok = Token::new(TokenType::Bang, &self.ch.to_string());
                }
            }
            '+' => tok = Token::new(TokenType::Plus, &self.ch.to_string()),
            '-' => tok = Token::new(TokenType::Minus, &self.ch.to_string()),
            '/' => tok = Token::new(TokenType::Slash, &self.ch.to_string()),
            '*' => tok = Token::new(TokenType::Asterisk, &self.ch.to_string()),
            '<' => tok = Token::new(TokenType::Lt, &self.ch.to_string()),
            '>' => tok = Token::new(TokenType::Gt, &self.ch.to_string()),
            '(' => tok = Token::new(TokenType::LParen, &self.ch.to_string()),
            ')' => tok = Token::new(TokenType::RParen, &self.ch.to_string()),
            ';' => tok = Token::new(TokenType::Semicolon, &self.ch.to_string()),
            ',' => tok = Token::new(TokenType::Comma, &self.ch.to_string()),
            '{' => tok = Token::new(TokenType::LBrace, &self.ch.to_string()),
            '}' => tok = Token::new(TokenType::RBrace, &self.ch.to_string()),
            '\0' => tok = Token::new(TokenType::Eof, &"".to_string()),
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = Token::lookup_ident(&literal);
                    tok = Token::new(token_type, literal);
                    return tok;
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    tok = Token::new(TokenType::Int, literal);
                    return tok;
                } else {
                    tok = Token::new(TokenType::Illegal, &self.ch.to_string())
                }
            }
        }

        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            let ch = self.input.chars().nth(self.read_position).unwrap();
            self.ch = ch;
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char()
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char()
        }
        &self.input[position..self.position]
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}
