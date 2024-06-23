use rustyline::DefaultEditor;

use crate::{lexer::Lexer, token::Token};

pub fn start() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let mut l = Lexer::new(&line);

                loop {
                    let tok = l.next_token();
                    if tok.token_type == Token::EOF {
                        break;
                    }
                    println!("{:?}", tok);
                }
            }
            Err(_) => {
                panic!();
            }
        }
    }
}
