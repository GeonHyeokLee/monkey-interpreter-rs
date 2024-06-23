use users::{get_current_uid, get_user_by_uid};

mod lexer;
mod repl;
mod token;

fn main() {
    let uid = get_current_uid();

    // UID를 이용해 유저 정보를 얻습니다.
    if let Some(user) = get_user_by_uid(uid) {
        println!(
            "\nHello {}! This is the Monkey programming language!",
            user.name().to_string_lossy()
        );
        println!("Feel free to type in commands");
        repl::start();
    } else {
        panic!()
    }
}
