mod lexer;
mod parser;
use std::io::{self,Write};
use lexer::tokenize;
use parser::parse;

fn main() {
    let mut input = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        if input.eq("exit"){
            break;
        }
        let tokens = tokenize(&input);
        let par = parse(&input);
        // for token in tokens{
            
        //     println!("{}",token);
        // }
        println!("{:?}",par);
        input.clear();
    }
}
