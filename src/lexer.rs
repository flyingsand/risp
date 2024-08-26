#[warn(dead_code)]

use std::error::Error;
use std::fmt::Display;
#[derive(Debug)]
pub struct LexerError {
    pub ch:char,
}
impl Error for LexerError {
    
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"unexpected character: {}",self.ch)
    }
}
#[derive(Debug,PartialEq, Eq,Clone)]
pub enum Token {
    LParen,
    RParen,
    Integer(i32),
    Symbol(String)
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "{}",i),
            Token::LParen => write!(f,"("),
            Token::RParen => write!(f,")"),
            Token::Symbol(s) => write!(f,"{}",s)
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token>{
    
    let words = input.replace("(", " ( ")
                             .replace(")", " ) ");
    let words = words.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();
    for word in words{
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ =>{
                let tok = word.parse::<i32>();
                match tok {
                    Ok(i) => tokens.push(Token::Integer(i)),
                    Err(_) => tokens.push(Token::Symbol(word.to_string()))
                }
            }
        }
    }
    tokens
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_add(){
        let tokens = tokenize("(+ 1 2)");
        assert_eq!(
        tokens,
        vec![
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Integer(1),
            Token::Integer(2),
            Token::RParen,
        ]
        );
    }
    

}