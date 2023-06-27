use std::collections::vec_deque;
use std::collections::VecDeque;

use atoi::atoi;
extern crate regex;

use super::bitsline as bl;
use super::formatter as fm;
use super::result::{Results, Line};

#[derive(PartialEq, Debug, Clone)]
pub enum BitwiseToken {
    AND,
    OR,
    XOR,
    RIGHTSHIFT,
    LEFTSHIFT,
    NOT,
    NUMBER,
    HEXNUMBER,
}

fn op_and(boi: &mut OperationInterpreter) -> Option<()> {
    let mut res_bits_line = bl::BitsLine::new(0);
    let nbr: [bl::BitsLine; 2] = [
        bl::BitsLine::new(boi.tokens[0].parse::<isize>().unwrap()),
        bl::BitsLine::new(boi.tokens[2].parse::<isize>().unwrap()),
    ];

    res_bits_line.update_values(nbr[0].value & nbr[1].value);

    boi.result.cache.append(&mut vec![
        ('=', res_bits_line.value),
        ('&', nbr[1].value),
        (' ', nbr[0].value),
    ]);

    boi.result.cache_x.append(&mut VecDeque::from([
        Line::Number(('=', res_bits_line.value)),
        Line::Under,
        Line::Number(('&', nbr[1].value)),
        Line::Number((' ', nbr[0].value)),
    ]));

    boi.nb_operations -= 1;
    Some(())
}

fn op_or(boi: &mut OperationInterpreter) -> Option<()> {
    let mut res_bits_line = bl::BitsLine::new(0);
    let nbr: [bl::BitsLine; 2] = [
        bl::BitsLine::new(boi.tokens[0].parse::<isize>().unwrap()),
        bl::BitsLine::new(boi.tokens[2].parse::<isize>().unwrap()),
    ];

    res_bits_line.update_values(nbr[0].value | nbr[1].value);

    boi.result.cache.append(&mut vec![
        ('=', res_bits_line.value),
        ('|', nbr[1].value),
        (' ', nbr[0].value),
    ]);
    boi.result.cache_x.append(&mut VecDeque::from([
        Line::Number(('=', res_bits_line.value)),
        Line::Under,
        Line::Number(('|', nbr[1].value)),
        Line::Number((' ', nbr[0].value)),
    ]));
    boi.nb_operations -= 1;
    Some(())
}

fn op_xor(boi: &mut OperationInterpreter) -> Option<()> {
    let mut res_bits_line = bl::BitsLine::new(0);
    let nbr: [bl::BitsLine; 2] = [
        bl::BitsLine::new(boi.tokens[0].parse::<isize>().unwrap()),
        bl::BitsLine::new(boi.tokens[2].parse::<isize>().unwrap()),
    ];

    res_bits_line.update_values(nbr[0].value ^ nbr[1].value);
    boi.result.cache_x.append(&mut VecDeque::from([
        Line::Number(('=', res_bits_line.value)),
        Line::Under,
        Line::Number(('^', nbr[1].value)),
        Line::Number((' ', nbr[0].value)),
    ]));
    boi.nb_operations -= 1;
    Some(())
}

// TODO: FIX NEGATIVE NUMBER
fn op_not(boi: &mut OperationInterpreter) -> String {
    let mut res_bits_line = bl::BitsLine::new(0);
    let nbr = bl::BitsLine::new(boi.tokens[1].parse::<isize>().unwrap());
    let int_res = !nbr.value;

    res_bits_line.update_values(int_res);
    boi.result.cache_x.append(&mut VecDeque::from([
        Line::Number(('=', res_bits_line.value)),
        Line::Under,
        Line::Number(('~', nbr.value)),
    ]));
    boi.nb_operations -= 1;
    int_res.to_string()
}

//fn op_rightshift(interpreter: &mut OperationInterpreter) -> Option<()> {
//let mut res_line = bl::BitsLine::new(interpreter.tokens[0].parse::<isize>().unwrap());
//let nb2 = bl::BitsLine::new(interpreter.tokens[2].parse::<isize>().unwrap());
//let op = fm::format_signed_output("   ", res_line.value);

//if nb2.value <= 0 {
//interpreter.result.push_front_res("Error: Cannot perform a rightshift with a value smaller than 1".to_string());
//interpreter.op_num -= 1;
//return None
//}
//res_line.update_values(res_line.value >> nb2.value);
//let res = fm::format_signed_output_shift(">>", res_line.value, nb2.value);
//let len_res = res.len();

//interpreter.result.push_front_res(res);
//if interpreter.op_num >= 1 {
//dump_line_of_len(&mut interpreter.result, len_res);
//}
//if interpreter.chains.0 != true {
//interpreter.result.push_front_res(op);
//}
//interpreter.op_num -= 1;
//Some(())
//}

//fn op_leftshift(interpreter: &mut OperationInterpreter) -> Option<()> {
//let mut res_line = bl::BitsLine::new(interpreter.tokens[0].parse::<isize>().unwrap());
//let nb2 = bl::BitsLine::new(interpreter.tokens[2].parse::<isize>().unwrap());
//let op = fm::format_signed_output("   ", res_line.value);

//if nb2.value <= 0 {
//interpreter.result.push_front_res("Error: Cannot perform a leftshift with a value smaller than 1".to_string());
//interpreter.op_num -= 1;
//return Some(());
//}
//res_line.update_values(res_line.value << nb2.value);
//let res = fm::format_signed_output_shift("<<", res_line.value, nb2.value);
//let len_res = res.len();

//interpreter.result.push_front_res(res);
//if interpreter.op_num >= 1 {
//dump_line_of_len(&mut interpreter.result, len_res);
//}
//if interpreter.chains.0 != true {
//interpreter.result.push_front_res(op);
//}
//interpreter.op_num -= 1;
//Some(())
//}

fn op_hexnumber(boi: &mut OperationInterpreter) -> Option<()> {
    // let res_bits_line = bl::BitsLine::new(boi.tokens[0].parse::<isize>().unwrap());
    let res = fm::format_hex_signed_output("✪", &boi.tokens[0]);
    boi.result.cache_x.append(&mut VecDeque::from([
        Line::Null,
    ]));
    boi.result.push_front_res(res);
    boi.result.push_front_res("#".to_string());
    boi.nb_operations -= 1;
    Some(())
}

fn op_number(boi: &mut OperationInterpreter) -> Option<()> {
    let res_bits_line = bl::BitsLine::new(boi.tokens[0].parse::<isize>().unwrap());
    
    boi.result.cache_x.append(&mut VecDeque::from([
        Line::Number(('✪', res_bits_line.value)),
    ]));
    Some(())
}

#[allow(dead_code)]
pub struct OperationInterpreter {
    tokens: Vec<String>,
    identifiers: Vec<BitwiseToken>,
    pub nb_operations: usize,
    pub result: Results,
    input: String,
    tmp_res: String,

    solo: bool,
    chains: (bool, usize),
}

impl OperationInterpreter {
    pub fn new() -> Self {
        OperationInterpreter {
            tokens: Vec::new(),
            identifiers: Vec::new(),
            nb_operations: 0,
            result: Results::new(),
            input: String::new(),
            tmp_res: String::new(),
            solo: false,
            chains: (false, 0),
        }
    }

    fn is_space(&self, c: char) -> bool {
        if c == ' ' || c == '\t' {
            return true;
        }
        false
    }

    fn is_hexval(&self, s: &str) -> bool {
        let a = s.chars().nth(0).unwrap();
        let b = s.chars().nth(1).unwrap();
        let c = s.chars().nth(2).unwrap();

        if (a == '-' && b == '0' && (c == 'x' || c == 'X')) || (a == '0' && (b == 'x' || b == 'X'))
        {
            return true;
        }
        false
    }

    fn get_hexval(&self, s: &str, i: &mut usize) -> String {
        let mut f = String::new();

        for ch in s.as_bytes() {
            // Checks for tabs or spaces
            if *ch == 0x20_u8 || *ch == 0x90_u8 {
                break;
            }
            *i += 1;
            f.push(*ch as char);
        }
        f
    }

    pub fn lexer(&mut self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut buffer: Vec<char> = Vec::new();
        let mut i: usize = 0;
        self.input = String::from(input);
        let input_len = input.len();
        let mut nbr: isize;

        while i < input_len {
            if self.is_space(input.chars().nth(i).unwrap()) == true {
                i += 1;
                continue;
            }
            // Check for hexvalues and get them
            if i + 2 < input_len && self.is_hexval(&input[i..]) == true {
                tokens.push(self.get_hexval(&input[i..], &mut i));
                continue;
            }

            // TODO: refacto
            if input.chars().nth(i).unwrap() == '-'
                && i + 1 < input.len()
                && input.chars().nth(i + 1).unwrap().is_ascii_digit()
                || input.chars().nth(i).unwrap().is_ascii_digit()
            {
                if !buffer.is_empty() {
                    tokens.push(buffer.iter().collect::<String>());
                    buffer.clear();
                }
                nbr = atoi::<isize>(input[i..].as_bytes()).unwrap();
                tokens.push(nbr.to_string());
                i += nbr.clone().to_string().len() - 1;
            } else {
                buffer.push(input.chars().nth(i).unwrap());
            }
            i += 1;
        }
        self.tokens = tokens.clone();
        tokens
    }

    fn init_number_operations(&mut self) {
        let corr_tokens = std::mem::take(&mut self.identifiers);

        corr_tokens.iter().for_each(|token| match token {
            BitwiseToken::NUMBER => {}
            _ => self.nb_operations += 1,
        });
        self.identifiers = corr_tokens;
    }

    pub fn clear_token_holders(&mut self) {
        self.identifiers.clear();
        self.tokens.clear();
    }

    // TODO: refactor with vetor methods
    pub fn chaining_checker(&mut self) -> (bool, usize) {
        let mut counter = 0;
        let mut not_num_counter = 0;
        let tmp = self.identifiers.clone();

        if tmp.len() == 2 && tmp[0] == BitwiseToken::NOT && tmp[1] == BitwiseToken::NUMBER {
            return (false, 0);
        }
        for token in tmp {
            if token == BitwiseToken::NUMBER || token == BitwiseToken::NOT {
                counter += 1;
            }
            if token != BitwiseToken::NUMBER && token != BitwiseToken::HEXNUMBER {
                not_num_counter += 1;
            }
        }
        if counter > 2 {
            return (true, not_num_counter);
        }
        (false, 0)
    }

    pub fn parser(&mut self) -> Result<(), String> {
        let mut is_err: bool = false;
        let mut corr_tokens = std::mem::take(&mut self.identifiers);
        let reg_hex = regex::Regex::new(r"^-?0[xX][0-9a-fA-F]+$").unwrap();

        self.tokens.clone().iter().for_each(|x| {
            let z: &str = &x;

            match z {
                "&" => corr_tokens.push(BitwiseToken::AND),
                "|" => corr_tokens.push(BitwiseToken::OR),
                "^" => corr_tokens.push(BitwiseToken::XOR),
                ">>" => corr_tokens.push(BitwiseToken::RIGHTSHIFT),
                "<<" => corr_tokens.push(BitwiseToken::LEFTSHIFT),
                "~" => corr_tokens.push(BitwiseToken::NOT),
                _ => {
                    if reg_hex.is_match(z) {
                        corr_tokens.push(BitwiseToken::HEXNUMBER)
                    } else if x.parse::<isize>().is_err() {
                        is_err = true;
                    } else {
                        corr_tokens.push(BitwiseToken::NUMBER);
                    }
                }
            }
        });
        self.identifiers = corr_tokens;
        // TODO: check if last token is valid as well as the first one
        let token_len = self.tokens.len();
        if is_err || token_len == 0 {
            let mut err: String = String::from("Error: cannot process this operation: ");
            err.push_str(&self.input);
            self.clear_token_holders();
            return Err(err);
        }
        self.chains = self.chaining_checker();
        self.init_number_operations();
        Ok(())
    }

    fn pop_off_tokens(&mut self, nbr: usize) {
        self.identifiers = self.identifiers[nbr..].to_vec();
        self.tokens = self.tokens[nbr..].to_vec();
    }

    pub fn interpreter(&mut self) {
        loop {
            match self.identifiers[..] {
                [] => {}
                [BitwiseToken::NOT, BitwiseToken::NUMBER, ..] => {
                    let mut shift = 1;
                    self.tmp_res = op_not(self);
                    if self.nb_operations == 0 {
                        let _ = std::mem::replace(&mut self.tokens[1], self.tmp_res.clone());
                        shift += 1;
                    }
                    self.pop_off_tokens(shift);
                }
                [BitwiseToken::NUMBER, BitwiseToken::AND, BitwiseToken::NUMBER, ..] => {
                    op_and(self).unwrap();
                    self.pop_off_tokens(3);
                }
                [BitwiseToken::NUMBER, BitwiseToken::OR, BitwiseToken::NUMBER, ..] => {
                    op_or(self).unwrap();
                    self.pop_off_tokens(3);
                }
                [BitwiseToken::NUMBER, BitwiseToken::XOR, BitwiseToken::NUMBER, ..] => {
                    op_xor(self).unwrap();
                    self.pop_off_tokens(3);
                }
                //[BitwiseToken::NUMBER, BitwiseToken::RIGHTSHIFT, BitwiseToken::NUMBER, ..] => {
                //let res = op_rightshift(self).unwrap();
                //self.corr_tokens = self.corr_tokens[3..].to_vec();
                //self.tokens = self.tokens[3..].to_vec();
                //},
                //[BitwiseToken::NUMBER, BitwiseToken::LEFTSHIFT, BitwiseToken::NUMBER, ..] => {
                //op_leftshift(self).unwrap();
                //self.corr_tokens = self.corr_tokens[3..].to_vec();
                //self.tokens = self.tokens[3..].to_vec();
                //},
                [BitwiseToken::NUMBER, ..] => {
                    op_number(self).unwrap();
                    self.pop_off_tokens(1);
                }
                [BitwiseToken::HEXNUMBER, ..] => {
                    op_hexnumber(self).unwrap();
                    self.pop_off_tokens(1);
                }
                _ => {}
            }
            if
            /* self.nb_operations == 0 &&  */
            self.identifiers.len() == 0 {
                break;
            }
        }
        self.result.format_result_cached();
    }
}

#[cfg(test)]
mod u_tests {
    use super::BitwiseToken;
    use super::OperationInterpreter;

    //#[test]
    //fn test_command_chaining() {
    //let input = "~12>>2<<3";
    //let mut op_interpreter = OperationInterpreter::new();

    //op_interpreter.lexer(input);
    //op_interpreter.parser().unwrap();
    //op_interpreter.interpreter();
    //assert_eq!("fpp", "vaa");
    //assert_eq!(op_interpreter.result.res,
    //["#", "~  12  00000000 00000000 00000000 00001100  0xc",
    //"───────────────────────────────────────────────",
    //"= -13  11111111 11111111 11111111 11110011 -0xd",
    //"────────────────────────────────────────────────",
    //">>2 -4  11111111 11111111 11111111 11111100 -0x4"]);
    //}

    //#[test]
    //fn test_mutliple_command_chain() {
    //let inputs =["~12>>2",
    //"0x12 123", "-1|3", "1     | 3  ",
    //"1 3", "-0x12", "0"];

    //let mut op_interpreter = OperationInterpreter::new();

    //for input in inputs {
    //op_interpreter.lexer(input);
    //op_interpreter.parser().unwrap();
    //op_interpreter.interpreter();
    //}
    //assert_ne!("fooo", "baar")
    //}

    //#[test]
    //fn test_mutliple_command_chain2() {
    //let input = "~12>>2";
    //let mut op_interpreter = OperationInterpreter::new();

    //op_interpreter.lexer(input);
    //op_interpreter.parser().unwrap();
    //op_interpreter.interpreter();

    //op_interpreter.lexer(input);
    //op_interpreter.parser().unwrap();
    //op_interpreter.interpreter();

    //assert_eq!(op_interpreter.result.res,
    //["#", "~  12  00000000 00000000 00000000 00001100  0xc",
    //"───────────────────────────────────────────────",
    //"= -13  11111111 11111111 11111111 11110011 -0xd",
    //"────────────────────────────────────────────────",
    //">>2 -4  11111111 11111111 11111111 11111100 -0x4"]);
    //}
    #[test]
    fn test_tokenizer() {
        let input = "123344abzefc2112333";
        let input2 = "0x12 123";
        let input3 = "-1|3";
        let input4 = "1     | 3  ";
        let input5 = "1 3";
        let input6 = "-0x12";
        let input7 = "0";
        let mut op_interpreter = OperationInterpreter::new();

        assert_eq!(
            ["123344", "abzefc", "2112333"].to_vec(),
            op_interpreter.lexer(input)
        );
        assert_eq!(["0x12", "123"].to_vec(), op_interpreter.lexer(input2));
        assert_eq!(["-1", "|", "3"].to_vec(), op_interpreter.lexer(input3));
        assert_eq!(["1", "|", "3"].to_vec(), op_interpreter.lexer(input4));
        assert_eq!(["1", "3"].to_vec(), op_interpreter.lexer(input5));
        assert_eq!(["-0x12"].to_vec(), op_interpreter.lexer(input6));
        assert_eq!(["0"].to_vec(), op_interpreter.lexer(input7));
    }

    #[test]
    fn test_parser_nums_only() {
        let num_only = "42";
        let hexnum_only = "0x2a";

        let mut op_interpreter = OperationInterpreter::new();
        op_interpreter.lexer(num_only);
        op_interpreter.parser().unwrap();
        assert_eq!(op_interpreter.identifiers, [BitwiseToken::NUMBER].to_vec());
        op_interpreter.interpreter();
        op_interpreter.lexer(hexnum_only);
        op_interpreter.parser().unwrap();
        assert_eq!(
            op_interpreter.identifiers,
            [BitwiseToken::HEXNUMBER].to_vec()
        );
    }

    #[test]
    fn test_invalid_input() {
        let input = "13a1";
        let input2 = "lorem ipsum";

        let mut op_interpreter = OperationInterpreter::new();
        op_interpreter.lexer(input);
        assert_eq!(
            op_interpreter.parser(),
            Err("Error: cannot process this operation: 13a1".to_string())
        );
        op_interpreter.input = input2.to_string();
        assert_eq!(
            op_interpreter.parser(),
            Err("Error: cannot process this operation: lorem ipsum".to_string())
        );
    }
    #[test]
    fn test_op_and() {
        let input = "12&13";
        let mut op_interpreter = OperationInterpreter::new();

        op_interpreter.lexer(input);
        op_interpreter.parser().unwrap();
        op_interpreter.interpreter();
        assert_eq!(
            op_interpreter.result.res,
            [
                "#",
                "   12  00001100  0xc",
                "&  13  00001101  0xd",
                "────────────────────",
                "=  12  00001100  0xc"
            ]
        );
    }

    #[test]
    fn test_op_or() {
        let input = "12|13";
        let mut op_interpreter = OperationInterpreter::new();

        op_interpreter.lexer(input);
        op_interpreter.parser().unwrap();
        op_interpreter.interpreter();
        assert_eq!(
            op_interpreter.result.res,
            [
                "#",
                "   12  00001100  0xc",
                "|  13  00001101  0xd",
                "────────────────────",
                "=  13  00001101  0xd"
            ]
        );
    }

    #[test]
    fn test_op_xor() {
        let input = "12^13";
        let mut op_interpreter = OperationInterpreter::new();

        op_interpreter.lexer(input);
        op_interpreter.parser().unwrap();
        op_interpreter.interpreter();
        assert_eq!(
            op_interpreter.result.res,
            [
                "#",
                "   12  00001100  0xc",
                "^  13  00001101  0xd",
                "────────────────────",
                "=   1  00000001  0x1"
            ]
        );
    }

    #[test]
    fn test_num_chained() {
        let input = "12 13 14";
        let mut op_interpreter = OperationInterpreter::new();

        op_interpreter.lexer(input);
        op_interpreter.parser().unwrap();
        op_interpreter.interpreter();
        assert_eq!(
            op_interpreter.result.res,
            [
                "#",
                "✪  14  00001110  0xe",
                "✪  13  00001101  0xd",
                "✪  12  00001100  0xc"
            ]
        );
    }

    #[test]
    fn test_hexnum_chained() { let input = "0x12 0x13 0x14"; let mut op_interpreter = OperationInterpreter::new(); op_interpreter.lexer(input); op_interpreter.parser().unwrap(); op_interpreter.interpreter(); assert_eq!(
            op_interpreter.result.res,
            [
                "#",
                "✪  0x14  00010100  20",
                "✪  0x13  00010011  19",
                "✪  0x12  00010010  18"
            ]
        );
    }

    #[test]
    fn test_op_not() {
        let input = "~12";
        let mut op_interpreter = OperationInterpreter::new();

        op_interpreter.lexer(input);
        op_interpreter.parser().unwrap();
        op_interpreter.interpreter();
        assert_eq!(
            op_interpreter.result.res,
            [
                "#",
                "~  12  00001100  0xc",
                "────────────────────",
                "= -13  11110011 -0xd"
            ]
        );
    }
}
