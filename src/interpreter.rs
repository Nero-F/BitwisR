use std::collections::{HashMap, VecDeque};
#[path = "lexer.rs"]
mod lexer;

use crate::lexer::{Lexer, Tokenv2};

pub struct Interpreter {
    buffer: Vec<Tokenv2>,
    pub postfix_expression: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            buffer: Vec::new(),
            postfix_expression: Vec::new(),
        }
    }

    pub fn parse(&mut self, input: String) {
        let mut tokenizer = Lexer::new(input);
        while let Ok(token) = tokenizer.next_token() {
            if let Tokenv2::EOF = token {
                break;
            }
            self.buffer.push(token);
        }
        self.infix_to_postfix();
    }

    fn prec(&self, tok: Tokenv2) -> i8 {
        match tok {
            Tokenv2::NOT => 4,
            Tokenv2::AND => 3,
            Tokenv2::XOR => 2,
            Tokenv2::OR => 1,
            _ => -1
        }
    }
    fn infix_to_postfix(&mut self) {
        let mut stack: VecDeque<Tokenv2> = VecDeque::new();
        let mut expression: VecDeque<&str> = VecDeque::with_capacity(self.buffer.len());

        let token_map: HashMap<Tokenv2, &str> = HashMap::from([
            (Tokenv2::AND, "&"),
            (Tokenv2::OR, "|"),
            (Tokenv2::XOR, "^"),
            (Tokenv2::RSHIFT, ">>"),
            (Tokenv2::LSHIFT, "<<"),
            (Tokenv2::RPARENT, ")"),
            (Tokenv2::LPARENT, "("),
            (Tokenv2::NOT, "~"),
        ]);

        for elem in &self.buffer {
            match elem {
                Tokenv2::NUMBER(x) => expression.push_back(&x),
                Tokenv2::HEXNUMBER(x) => expression.push_back(&x),
                Tokenv2::LPARENT => stack.push_back(elem.to_owned()),
                Tokenv2::RPARENT => {
                    while stack.back() != Some(&Tokenv2::LPARENT) {
                        expression.push_back(token_map[stack.back().unwrap()]);
                        stack.pop_back();
                    }
                    stack.pop_back();
                }
                Tokenv2::AND
                | Tokenv2::OR
                | Tokenv2::XOR
                | Tokenv2::NOT
                | Tokenv2::RSHIFT
                | Tokenv2::LSHIFT => {
                    while !stack.is_empty()
                        && self.prec(elem.to_owned()) <= self.prec(stack.back().unwrap().to_owned())
                    {
                        expression.push_back(token_map[stack.back().unwrap()]);
                        stack.pop_back();
                    }
                    stack.push_back(elem.to_owned());
                }
                _ => {}
            }
        }
        while !stack.is_empty() {
            expression.push_back(token_map[stack.back().unwrap()]);
            stack.pop_back();
        }
        self.postfix_expression = expression.iter().map(|&s|s.into()).collect();
    }
}
