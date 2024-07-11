use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    Lt,
    Gt,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
}

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("fn", TokenType::Function);
    m.insert("let", TokenType::Let);
    m.insert("true", TokenType::True);
    m.insert("false", TokenType::False);
    m.insert("if", TokenType::If);
    m.insert("else", TokenType::Else);
    m.insert("return", TokenType::Return);
    m
});

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenType {
        let token = KEYWORDS.get(ident).cloned();

        match token {
            Some(token) => token,
            None => TokenType::Ident,
        }
    }
}
