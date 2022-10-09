mod lexer;

extern crate bitter_oyster;
/*
#[allow(unused_imports)] // allowing the unused imports
use std::fs;
use std::env;

#[derive(Debug)]

struct Lexer {
    contents: String,

}

impl Lexer {
    pub fn new(contents:String) -> Self{
         // implicit return in rust 
            Self { contents }
    }
}
*/

fn main() {
       let input = String::from("let a = 5;");
    let mut l = lexer::Lexer::new(input.chars().collect());
    l.read_char();
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EOF {
            break;
        } else {
            println!("{:?}", token);
        }
    }
    println!("{} {} {}", char::from(l.ch), l.position, l.read_position);
}
    
    
    /*

    
    let file = env::args().nth(1).unwrap();

    let contents = fs::read_to_string(file).unwrap();

    let lexer = Lexer::new(contents);

    
    println!("{:?}",lexer);*/

