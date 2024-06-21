use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    fn peekChar(&mut self) -> char {
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

    fn next_token(&mut self) -> Token {
        let tok: Token;

        match self.ch {
            '=' => {
                if self.peekChar() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token::new(Token::EQ, literal);
                } else {
                    tok = Token::new(Token::ASSIGN, self.ch.to_string());
                }
            }
            '!' => {
                if self.peekChar() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token::new(Token::NOT_EQ, literal);
                } else {
                    tok = Token::new(Token::BANG, self.ch.to_string());
                }
            }
            '+' => tok = Token::new(Token::PLUS, self.ch.to_string()),
            '-' => tok = Token::new(Token::MINUS, self.ch.to_string()),
            '/' => tok = Token::new(Token::SLASH, self.ch.to_string()),
            '*' => tok = Token::new(Token::ASTERISK, self.ch.to_string()),
            '<' => tok = Token::new(Token::LT, self.ch.to_string()),
            '>' => tok = Token::new(Token::GT, self.ch.to_string()),
            '(' => tok = Token::new(Token::LPAREN, self.ch.to_string()),
            ')' => tok = Token::new(Token::RPAREN, self.ch.to_string()),
            ';' => tok = Token::new(Token::SEMICOLON, self.ch.to_string()),
            ',' => tok = Token::new(Token::COMMA, self.ch.to_string()),
            '{' => tok = Token::new(Token::LBRACE, self.ch.to_string()),
            '}' => tok = Token::new(Token::RBRACE, self.ch.to_string()),
            '\0' => tok = Token::new(Token::EOF, "".to_string()),
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = Token::lookup_ident(&literal);
                    tok = Token::new(&token_type, literal)
                } else if Lexer::is_digit(self.ch) {
                    tok = Token::new(Token::INT, self.read_number())
                } else {
                    tok = Token::new(Token::ILLEGAL, self.ch.to_string());
                }
            }
        }

        self.read_char();
        tok
    }
}