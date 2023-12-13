#[path = "lexer.rs"]
mod lexer;

use crate::lexer::{Lexer, Tokenv2};

pub struct Interpreter {
    pub buffer: Vec<Tokenv2>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { buffer: Vec::new() }
    }

    pub fn parse(&mut self, input: String) {
        let mut tokenizer = Lexer::new(input);
        while let Ok(token) = tokenizer.next_token() {
            if let Tokenv2::EOF = token {
                break;
            }
            self.buffer.push(token);
        }
    }
}
