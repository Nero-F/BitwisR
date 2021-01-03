use atoi::atoi;
use std::collections::HashMap;

#[path = "bitsline.rs"]
mod bl;
#[path = "result.rs"]
mod result;

#[derive(PartialEq)]
pub enum BitwiseToken {
    AND,
    OR,
    XOR,
    RIGHTSHIFT,
    LEFTSHIFT,
    NOT,
    NUMBER
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

//fn op_leftshift(interpreter: &mut OperationInterpreter) -> Option<()> {
    //Some(())
//}


// TODO: negative values input
fn op_not(interpreter: &mut OperationInterpreter) -> Option<()> {
    let mut res_line = bl::BitsLine::new(interpreter.tokens[1].parse::<isize>().unwrap());
    let op = format!("{}  {} {} {:#x}", interpreter.tokens[0], res_line.value, format_bin_string(res_line.value), res_line.value);
    res_line.update_values(!res_line.value);
    let res = format!("= {} {} -{:#x}", res_line.value, format_bin_string(res_line.value),  -1 * res_line.value);
    let len_res = res.len();

    interpreter.result.push_front_res(res);
    if interpreter.op_num == 1 {
        let line = (0..len_res).map(|_| "─").collect::<String>();
        interpreter.result.push_front_res(line);
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
    pub result: result::Results
}

impl OperationInterpreter {
    pub fn new() -> Self {
        OperationInterpreter { op_func: op_mapper(), tokens: Vec::new(), corr_tokens: Vec::new(), op_num: 0, result: result::Results::new() }
    }

    pub fn lexer(&mut self, input: &str) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut buffer: Vec<char> = Vec::new();
        let mut nbr: isize;
        let mut i: usize = 0;
        let epur = input.chars().filter(|c| !c.is_whitespace()).collect::<String>();

        while i < epur.len() {
            if epur.chars().nth(i).unwrap().is_ascii_digit() {
                if !buffer.is_empty() {
                    tokens.push(buffer.iter().collect::<String>());
                    buffer.clear();
                }
                nbr = atoi::<isize>(epur[i..].as_bytes()).unwrap();
                tokens.push(nbr.to_string());
                i += nbr.clone().to_string().len() - 1;
            } else {
                buffer.push(epur.chars().nth(i).unwrap());
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

    pub fn parser(&mut self) -> Result<(), &str> {
        let mut is_err: bool = false;
        let mut corr_tokens = std::mem::take(&mut self.corr_tokens);

        self.tokens
            .clone()
            .iter()
            .for_each(|x| {
                let z: &str = &x;

                match z {
                    "&" => corr_tokens.push(BitwiseToken::AND),
                    "|" => corr_tokens.push(BitwiseToken::OR),
                    "^" => corr_tokens.push(BitwiseToken::XOR),
                    ">>" => corr_tokens.push(BitwiseToken::RIGHTSHIFT),
                    "<<" => corr_tokens.push(BitwiseToken::LEFTSHIFT),
                    "~" => corr_tokens.push(BitwiseToken::NOT),
                    _ => { 
                        if x.parse::<isize>().is_err() {
                            is_err = true;
                        } else {
                            corr_tokens.push(BitwiseToken::NUMBER);
                        }
                    }
                }
            });
        self.corr_tokens = corr_tokens;
        self.init_op_number();
        if is_err {
            let mut err: String = String::from("Cannot process this operation: ");
            err.push_str(&self.tokens.clone().join(""));
            panic!(err);
        }
        Ok(())
    }

    pub fn interpreter(&mut self) {
        match self.corr_tokens[..] {
            [] => {},
            [BitwiseToken::NOT, BitwiseToken::NUMBER] => op_not(self).unwrap(),
            _ => {},
        }

        self.corr_tokens.clear();
        self.tokens.clear();
    }
}

#[test]
fn test_tokenizer() {
    let input = "123344abzefc2112333";
    let input2 = "azda1231azdaz11";
    let input3 = "1|3";
    let input4 = "1     | 3  ";
    let mut op_interpreter = OperationInterpreter::new();

    assert_eq!(["123344", "abzefc", "2112333"].to_vec(), op_interpreter.lexer(input));
    assert_eq!(["azda", "1231", "azdaz", "11"].to_vec(), op_interpreter.lexer(input2));
    assert_eq!(["1", "|", "3"].to_vec(), op_interpreter.lexer(input3));
    assert_eq!(["1", "|", "3"].to_vec(), op_interpreter.lexer(input4));
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
