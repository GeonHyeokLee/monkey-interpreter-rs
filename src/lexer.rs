use crate::token::Token;

#[test]
fn test_next_token() {
    // const INPUT: &str = "let five = 5;";
    const INPUT: &str = "5;";

    let mut tests = Vec::new();
    // tests.push([Token::LET, "let"]);
    // tests.push([Token::IDENT, "five"]);
    // tests.push([Token::ASSIGN, "="]);
    // tests.push([Token::INT, "5"]);
    // tests.push([Token::SEMICOLON, ";"]);
    // tests.push([Token::EOF, ""]);

    tests.push([Token::INT, "5"]);
    tests.push([Token::SEMICOLON, ";"]);
    tests.push([Token::EOF, ""]);

    let mut l = Lexer::new(INPUT);

    for (i, tt) in tests.iter().enumerate() {
        let tok = l.next_token();
        assert_eq!(
            tok.token_type, tt[0],
            "tests[{}] - tokentype wrong. expected={}, got={}",
            i, tt[0], tok.token_type
        );
        assert_eq!(
            tok.literal, tt[1],
            "tests[{}] - literal wrong. expected={}, got={}",
            i, tt[1], tok.literal
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

    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token::new(Token::EQ, &literal);
                } else {
                    tok = Token::new(Token::ASSIGN, &self.ch.to_string());
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = ch.to_string() + &self.ch.to_string();
                    tok = Token::new(Token::NOT_EQ, &literal);
                } else {
                    tok = Token::new(Token::BANG, &self.ch.to_string());
                }
            }
            '+' => tok = Token::new(Token::PLUS, &self.ch.to_string()),
            '-' => tok = Token::new(Token::MINUS, &self.ch.to_string()),
            '/' => tok = Token::new(Token::SLASH, &self.ch.to_string()),
            '*' => tok = Token::new(Token::ASTERISK, &self.ch.to_string()),
            '<' => tok = Token::new(Token::LT, &self.ch.to_string()),
            '>' => tok = Token::new(Token::GT, &self.ch.to_string()),
            '(' => tok = Token::new(Token::LPAREN, &self.ch.to_string()),
            ')' => tok = Token::new(Token::RPAREN, &self.ch.to_string()),
            ';' => tok = Token::new(Token::SEMICOLON, &self.ch.to_string()),
            ',' => tok = Token::new(Token::COMMA, &self.ch.to_string()),
            '{' => tok = Token::new(Token::LBRACE, &self.ch.to_string()),
            '}' => tok = Token::new(Token::RBRACE, &self.ch.to_string()),
            '\0' => tok = Token::new(Token::EOF, &"".to_string()),
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = Token::lookup_ident(&literal);
                    tok = Token::new(token_type, literal);
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    tok = Token::new(Token::INT, literal);
                } else {
                    tok = Token::new(Token::ILLEGAL, &self.ch.to_string());
                }
            }
        }

        self.read_char();
        tok
    }
}
