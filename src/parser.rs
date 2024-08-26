use super::lexer::*;
#[derive(Debug,PartialEq, Eq)]
pub enum Object {
    Void,
    Integer(i32),
    Bool(bool),
    Symbol(String),
    List(Vec<Object>),
    Lambda(Vec<String>,Vec<Object>)
}



pub fn parse(program:&str)->Object{
    let mut tokens = tokenize(program).into_iter().rev().collect::<Vec<Token>>();
    parse_list(&mut tokens)
}

pub fn parse_list(tokens:&mut Vec<Token>)->Object{
    let first_token = tokens.pop();
    
    match first_token {
        Some(Token::LParen) =>{},
        _ =>{},
    }
    let mut list:Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if let Some(token) =token  {
            match token {
                Token::LParen => {
                    tokens.push(Token::LParen);
                    list.push(parse_list(tokens));
                },
                Token::RParen =>{
                    return Object::List(list);
                },
                Token::Integer(i) =>{
                    list.push(Object::Integer(i));
                },
                Token::Symbol(s) =>{
                    list.push(Object::Symbol(s));
                }
            }
        }
    }

    Object::List(list)
}