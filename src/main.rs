mod lexer;
use std::io::{self,Write};
use lexer::tokenize;
fn main() {
    let mut input = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        if input == "exit".to_string(){
            break;
        }
        let tokens = tokenize(&input);
        for token in tokens{
            
            println!("{}",token);
    }
        input.clear();
    }
}
