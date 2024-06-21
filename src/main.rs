use lexer::Lexer;

mod lexer;
mod token;

fn main() {
    const INPUT: &str = "let five = 5";

    let l = Lexer::new(INPUT.to_string());

    println!("{:?}", l);
}
