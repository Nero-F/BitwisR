use atoi::atoi;
use std::collections::HashMap;
extern crate regex;

#[path = "bitsline.rs"]
mod bl;
#[path = "result.rs"]
mod result;

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

type Callback = fn(&mut OperationInterpreter) -> Option<()>;

//TODO: handle 64bits values
fn format_bin_string(value: isize) -> String {
    let mut well_formated: Vec<char> = Vec::new();
    let tmp: Vec<char> = format!("{:064b}", value).chars().collect();
    let limit = if value > 4294967295 { 64 } else { 32 };
    let mut end_tmp: usize = tmp.len()-1;

    for i in (0..limit).rev() {
        well_formated.insert(0, tmp[end_tmp]);
        if i % 8 == 0 {
            well_formated.insert(0, ' ');
        }
        end_tmp -= 1;
    }
    well_formated.iter().collect::<String>()
}

fn dump_line_of_len(result: &mut result::Results, length: usize) {
    let line = (0..length).map(|_| "─").collect::<String>();
    result.push_front_res(line);
}

//fn op_and(interpreter: &mut OperationInterpreter) -> Option<()> {
    //Some(())
//}

//fn op_or(interpreter: &mut OperationInterpreter) -> Option<()> {
    //Some(())
//}

//fn op_xor(interpreter: &mut OperationInterpreter) -> Option<()> {
    //Some(())
//}

//fn op_rightshift(interpreter: &mut OperationInterpreter) -> Option<()> {
    //Some(())
//}

fn format_signed_output_shift(token: &str, value: isize, value2: isize) -> String {
    if value < 0 {
        format!("{}{} -{} {} -{:#x}", token, value2, -1 * value, format_bin_string(value), -1 * value)
    } else {
        format!("{}{}  {} {}  {:#x}", token, value2, value, format_bin_string(value), value)
    }
}

fn format_signed_output(token: &str, v: isize) -> String {
    if v < 0 {
        format!("{} -{} {} -{:#x}", token, -1 * v, format_bin_string(v), -1 * v)
    } else {
        format!("{}  {} {}  {:#x}", token, v, format_bin_string(v), v)
    }
}

fn format_hex_signed_output(token: &str, h: &str) -> String {
    let hl = h.to_lowercase();
    let mut neg = false;

    let trimmed = if h.chars().nth(0).unwrap() == '-' {
        neg = true;
        hl.trim_start_matches("-0x")
    } else {
        hl.trim_start_matches("0x")
    };
    let v = isize::from_str_radix(trimmed, 16).unwrap();

    if neg == true {
        format!("{} {} {} {}", token, hl, format_bin_string(-1 * v), -1 * v)
    } else {
        format!("{}  {} {}  {}", token, hl, format_bin_string(v), v)
    }
}

fn op_leftshift(interpreter: &mut OperationInterpreter) -> Option<()> {
    let mut res_line = bl::BitsLine::new(interpreter.tokens[0].parse::<isize>().unwrap());
    let nb2 = bl::BitsLine::new(interpreter.tokens[2].parse::<isize>().unwrap());
    let op = format_signed_output("   ", res_line.value);
    
    if nb2.value <= 0 {
        interpreter.result.push_front_res("Error: Cannot perform a leftshift with a value smaller than 1".to_string());
        interpreter.op_num -= 1;
        return Some(());
    }
    res_line.update_values(res_line.value << nb2.value);
    let res = format_signed_output_shift("<<", res_line.value, nb2.value);
    let len_res = res.len();

    interpreter.result.push_front_res(res);
    if interpreter.op_num >= 1 {
        dump_line_of_len(&mut interpreter.result, len_res);
    }
    interpreter.result.push_front_res(op);
    interpreter.op_num -= 1;
    Some(())
}

fn op_hexnumber(interpreter: &mut OperationInterpreter) -> Option<()> {
    let res = format_hex_signed_output("✪", &interpreter.tokens[0]);
    interpreter.result.push_front_res(res);
    Some(())
}

fn op_number(interpreter: &mut OperationInterpreter) -> Option<()> {
    let res_line = bl::BitsLine::new(interpreter.tokens[0].parse::<isize>().unwrap());
    let res = format_signed_output("✪", res_line.value);
    interpreter.result.push_front_res(res);
    Some(())
}

fn op_not(interpreter: &mut OperationInterpreter) -> Option<()> {
    let mut res_line = bl::BitsLine::new(interpreter.tokens[1].parse::<isize>().unwrap());
    let op = format_signed_output(&interpreter.tokens[0], res_line.value);
    res_line.update_values(!res_line.value);
    let res = format_signed_output("=", res_line.value);
    let len_res = res.len();

    interpreter.result.push_front_res(res);
    if interpreter.op_num >= 1 {
        dump_line_of_len(&mut interpreter.result, len_res);
    }
    interpreter.result.push_front_res(op);
    interpreter.op_num -= 1;
    interpreter.result.push_front_res("#".to_string());
    Some(())
}

fn op_mapper() -> HashMap<String, Callback> {
    let mut map: HashMap<String, Callback> = HashMap::new();

    //map.insert("&".to_string(), op_and);
    //map.insert("|".to_string(), op_or);
    //map.insert("^".to_string(), op_xor);
    //map.insert(">>".to_string(), op_rightshift);
    //map.insert("<<".to_string(), op_leftshift);
    map.insert("~".to_string(), op_not);
    map
}

pub struct OperationInterpreter {
    op_func: HashMap<String, Callback>,
    tokens: Vec<String>,
    corr_tokens: Vec<BitwiseToken>,
    pub op_num: usize,
    pub result: result::Results,
    input: String
}

impl OperationInterpreter {
    pub fn new() -> Self {
        OperationInterpreter { op_func: op_mapper(), tokens: Vec::new(), corr_tokens: Vec::new(), op_num: 0, result: result::Results::new(), input: String::new() }
    }

    pub fn lexer(&mut self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut buffer: Vec<char> = Vec::new();
        let mut nbr: isize;
        let mut i: usize = 0;
        self.input = String::from(input);

        while i < input.len() {
            // check for spaces
            if input.chars().nth(i).unwrap() == ' ' ||
               input.chars().nth(i).unwrap() == '\t' {
                i += 1;
                continue;
            }
            // Check for hexvalues and get them
            // TODO: Maybe replace this with regex
            if i + 2 < input.len() &&
               ((input.chars().nth(i).unwrap() == '-' && input.chars().nth(i+1).unwrap() == '0' &&
               (input.chars().nth(i+2).unwrap() == 'x' || input.chars().nth(i+2).unwrap() == 'X')) ||
               (input.chars().nth(i).unwrap() == '0' &&
               (input.chars().nth(i+1).unwrap() == 'x' || input.chars().nth(i+1).unwrap() == 'X'))) {
                let mut f = String::new();
                for ch in input[i..].as_bytes() {
                    // Checks for tabs or spaces
                    if *ch == 0x20_u8 || *ch == 0x90_u8 {
                        break;
                    }
                    i += 1;
                    f.push(*ch as char);
                }
                tokens.push(f);
                continue;
            }
            // Check for decimal values and get them
            if input.chars().nth(i).unwrap() == '-' && i + 1 < input.len() &&
               input.chars().nth(i+1).unwrap().is_ascii_digit() ||
               input.chars().nth(i).unwrap().is_ascii_digit()
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

    fn init_op_number(&mut self) {
        let corr_tokens = std::mem::take(&mut self.corr_tokens);

        corr_tokens
            .iter()
            .for_each(|token| {
                match token {
                    BitwiseToken::NUMBER => {},
                    _ => self.op_num += 1
                }
            });
        self.corr_tokens = corr_tokens;
    }

    pub fn clear_token_holders(&mut self) {
        self.corr_tokens.clear();
        self.tokens.clear();
    }

    pub fn parser(&mut self) -> Result<(), String> {
        let mut is_err: bool = false;
        let mut corr_tokens = std::mem::take(&mut self.corr_tokens);
        let reg_hex = regex::Regex::new(r"^-?0[xX][0-9a-fA-F]+$").unwrap();

        self.tokens
            .clone()
            .iter()
            .for_each(|x| {
                let z: &str = &x;

                match z {
                    "&" => corr_tokens.push(BitwiseToken::AND),
                    "|" => {
                        corr_tokens.push(BitwiseToken::OR);
                        is_err = true;
                    },
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
        self.corr_tokens = corr_tokens;
        if is_err || self.tokens.len() == 0 {
            let mut err: String = String::from("Error: cannot process this operation: ");
            err.push_str(&self.input);
            self.clear_token_holders();
            return Err(err);
        }
        self.init_op_number();
        Ok(())
    }

    pub fn interpreter(&mut self) {
        match self.corr_tokens[..] {
            [] => {},
            [BitwiseToken::NUMBER, BitwiseToken::LEFTSHIFT, BitwiseToken::NUMBER, ..] => {
                op_leftshift(self).unwrap();
                self.corr_tokens = self.corr_tokens[3..].to_vec();
                self.tokens = self.tokens[3..].to_vec();
            }
            [BitwiseToken::NOT, BitwiseToken::NUMBER, ..] => {
                op_not(self).unwrap();
                self.corr_tokens = self.corr_tokens[2..].to_vec();
                self.tokens = self.tokens[2..].to_vec();
            },
            [BitwiseToken::NUMBER, ..] => {
                op_number(self).unwrap();
                self.corr_tokens = self.corr_tokens[1..].to_vec();
                self.tokens = self.tokens[1..].to_vec();
            },
            [BitwiseToken::HEXNUMBER, ..] => {
                op_hexnumber(self).unwrap();
                self.corr_tokens = self.corr_tokens[1..].to_vec();
                self.tokens = self.tokens[1..].to_vec();
            },
            _ => {},
        }
        if self.corr_tokens.len() != 0 {
            self.interpreter();
            return;
        }
        self.clear_token_holders();
        self.result.push_front_res("#".to_string());
    }
}

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

    assert_eq!(["123344", "abzefc", "2112333"].to_vec(), op_interpreter.lexer(input));
    assert_eq!(["0x12", "123"].to_vec(), op_interpreter.lexer(input2));
    assert_eq!(["-1", "|", "3"].to_vec(), op_interpreter.lexer(input3));
    assert_eq!(["1", "|", "3"].to_vec(), op_interpreter.lexer(input4));
    assert_eq!(["1", "3"].to_vec(), op_interpreter.lexer(input5));
    assert_eq!(["-0x12"].to_vec(), op_interpreter.lexer(input6));
    assert_eq!(["0"].to_vec(), op_interpreter.lexer(input7));
}

#[test]
fn test_parser() {
    let num_only = "42";
    let hexnum_only = "0x2a";

    let mut op_interpreter = OperationInterpreter::new();
    op_interpreter.lexer(num_only);
    op_interpreter.parser().unwrap();
    assert_eq!(op_interpreter.corr_tokens, [BitwiseToken::NUMBER].to_vec());
    op_interpreter.interpreter();
    op_interpreter.lexer(hexnum_only);
    op_interpreter.parser().unwrap();
    assert_eq!(op_interpreter.corr_tokens, [BitwiseToken::HEXNUMBER].to_vec());
    op_interpreter.interpreter();
}

#[test]
fn test_invalid_input() {
    let input = "13a1";
    let input2 = "lorem ipsum";

    let mut op_interpreter = OperationInterpreter::new();
    op_interpreter.lexer(input);
    assert_eq!(op_interpreter.parser(), Err("Error: cannot process this operation: 13a1".to_string()));
    op_interpreter.input = input2.to_string();
    assert_eq!(op_interpreter.parser(), Err("Error: cannot process this operation: lorem ipsum".to_string()));
}

#[test]
fn test_format_bin_string() {
    let value = 1;
    let value2 = 255;
    let value3 = 65535;
    let value4 = 16777215;
    let value5 = 2147483647;
    let value6 = 2147483648;
    let value7: isize = 2147483649;

    assert_eq!(" 00000000 00000000 00000000 00000001", format_bin_string(value));
    assert_eq!(" 00000000 00000000 00000000 11111111", format_bin_string(value2));
    assert_eq!(" 00000000 00000000 11111111 11111111", format_bin_string(value3));
    assert_eq!(" 00000000 11111111 11111111 11111111", format_bin_string(value4));
    assert_eq!(" 01111111 11111111 11111111 11111111", format_bin_string(value5));
    assert_eq!(" 10000000 00000000 00000000 00000000", format_bin_string(value6));
    assert_eq!(" 10000000 00000000 00000000 00000001", format_bin_string(value7));
}
