use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Token {
    pub token_type: String,
    pub literal: String,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("fn", Token::FUNCTION);
        m.insert("let", Token::LET);
        m.insert("true", Token::TRUE);
        m.insert("false", Token::FALSE);
        m.insert("if", Token::IF);
        m.insert("else", Token::ELSE);
        m.insert("return", Token::RETURN);
        m
    };
}

impl Token {
    pub const ILLEGAL: &'static str = "ILLEGAL";
    pub const EOF: &'static str = "EOF";
    pub const IDENT: &'static str = "IDENT";
    pub const INT: &'static str = "INT";
    pub const ASSIGN: &'static str = "=";
    pub const PLUS: &'static str = "+";
    pub const MINUS: &'static str = "-";
    pub const BANG: &'static str = "!";
    pub const ASTERISK: &'static str = "*";
    pub const SLASH: &'static str = "/";
    pub const COMMA: &'static str = ";";
    pub const SEMICOLON: &'static str = ";";
    pub const LT: &'static str = "<";
    pub const GT: &'static str = ">";
    pub const LPAREN: &'static str = "(";
    pub const RPAREN: &'static str = ")";
    pub const LBRACE: &'static str = "{";
    pub const RBRACE: &'static str = "}";
    pub const FUNCTION: &'static str = "FUNCTION";
    pub const LET: &'static str = "LET";
    pub const TRUE: &'static str = "TRUE";
    pub const FALSE: &'static str = "FALSE";
    pub const IF: &'static str = "IF";
    pub const ELSE: &'static str = "ELSE";
    pub const RETURN: &'static str = "RETURN";
    pub const EQ: &'static str = "==";
    pub const NOT_EQ: &'static str = "!=";

    pub fn new(token_type: &str, literal: &str) -> Token {
        Token {
            token_type: token_type.to_string(),
            literal: literal.to_string(),
        }
    }

    pub fn lookup_ident(ident: &str) -> &str {
        let token = KEYWORDS.get(ident);

        match token {
            Some(token) => token,
            None => Token::IDENT,
        }
    }
}
