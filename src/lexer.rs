use anyhow::Result;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Tokenv2 {
    AND,
    OR,
    XOR,
    RIGHTSHIFT,
    LEFTSHIFT,
    NOT,
    EOF,
    ILLEGAL,
    NUMBER(String),
    HEXNUMBER(String),
}

impl Display for Tokenv2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Tokenv2::AND => write!(f, "AND"),
            Tokenv2::OR => write!(f, "OR"),
            Tokenv2::XOR => write!(f, "XOR"),
            Tokenv2::RIGHTSHIFT => write!(f, "RIGHTSHIFT"),
            Tokenv2::LEFTSHIFT => write!(f, "LEFTSHIFT"),
            Tokenv2::NOT => write!(f, "NOT"),
            Tokenv2::EOF => write!(f, "Eof"),
            Tokenv2::ILLEGAL => write!(f, "ILLEGAL"),
            Tokenv2::NUMBER(x) => write!(f, "NUMBER({})", x),
            Tokenv2::HEXNUMBER(x) => write!(f, "HEXNUMBER({})", x),
        };
    }
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lex.read_char();
        return lex;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        return self.input[self.read_position];
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
    fn read_hex_number(&mut self) -> String {
        let pos = self.position;

        while self.ch.is_ascii_hexdigit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn skip_spaces(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Result<Tokenv2> {
        self.skip_spaces();

        let token: Tokenv2 = match self.ch {
            b'&' => Tokenv2::AND,
            b'|' => Tokenv2::OR,
            b'^' => Tokenv2::XOR,
            b'!' => Tokenv2::NOT,
            b'>' => {
                if self.peek_char() != b'>' {
                    unreachable!("PPAS OUF")
                }
                self.read_char();
                Tokenv2::RIGHTSHIFT
            }
            b'<' => {
                if self.peek_char() != b'<' {
                    unreachable!("PPAS OUF")
                }
                self.read_char();
                Tokenv2::LEFTSHIFT
            }
            b'0'..=b'9' => {
                if self.ch == b'0' && self.peek_char() == b'X' |/* | self.peek_char() == */ b'x' {
                    self.read_char();
                    self.read_char();
                    Tokenv2::HEXNUMBER(self.read_hex_number())
                } else {
                    Tokenv2::NUMBER(self.read_number())
                }
            }
            0 => Tokenv2::EOF,
            _ => {
                unreachable!("PPAS OUF")
            }
        };

        self.read_char();

        return Ok(token);
    }
}

#[test]
fn get_next_token() -> Result<()> {
    let input = "&|^!>><< 42 0x12";
    let mut lexer = Lexer::new(input.into());

    let tokens = vec![
        Tokenv2::AND,
        Tokenv2::OR,
        Tokenv2::XOR,
        Tokenv2::NOT,
        Tokenv2::RIGHTSHIFT,
        Tokenv2::LEFTSHIFT,
        Tokenv2::NUMBER("42".to_string()),
        Tokenv2::HEXNUMBER("12".to_string()),
    ];

    for token in tokens {
        let next_token = lexer.next_token()?;
        println!("expected: {:?}, received {:?}", token, next_token);
        assert_eq!(token, next_token);
    }
    return Ok(());
}
