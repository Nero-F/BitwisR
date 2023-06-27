use std::collections::VecDeque;

use super::binary_operation::BitwiseToken;

pub struct Tokens {
    values: VecDeque<String>,
    identifiers: VecDeque<BitwiseToken>,
}

impl Tokens {
    pub fn new() -> Self {
        Tokens {
            values: VecDeque::new(),
            identifiers: VecDeque::new(),
        }
    }

    pub fn push_value(&mut self, value: String) {
        self.values.push_back(value);
    }
    pub fn push_identifier(&mut self, identifier: BitwiseToken) {
        self.identifiers.push_back(identifier);
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.identifiers.clear();
    }
}
