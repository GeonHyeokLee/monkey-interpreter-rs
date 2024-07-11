use std::process;

use rustyline::DefaultEditor;

use crate::{lexer::Lexer, token::TokenType};

pub fn start() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let mut l = Lexer::new(&line);

                loop {
                    let tok = l.next_token();
                    if tok.token_type == TokenType::Eof {
                        break;
                    }
                    println!("{:?}", tok);
                }
            }
            Err(_) => {
                process::exit(0);
            }
        }
    }
}
