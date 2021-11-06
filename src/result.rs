use tuikit::prelude::*;

#[path = "formatter.rs"]
mod fm;

type ResultLine = (char, isize);

#[allow(dead_code)]
pub struct Results {
    history: Vec<String>,
    pub res: Vec<String>,
    pub tmp_buff: Vec<String>,
    pub cache: Vec<ResultLine>,
}

#[allow(dead_code)]
impl Results {
    pub fn new() -> Self {
        Results { history: Vec::new(), res: Vec::new(), tmp_buff: Vec::new(), cache: Vec::new()}
    }

    fn is_only_space(&self, result: String) -> bool {
        for ch in result.chars() {
            if ch != ' ' {
                return false;
            }
        }
        true
    }

    pub fn add_to_history(&mut self, result: String) {
        if !result.is_empty() && !self.is_only_space(result.clone()) {
            self.history.push(result);
        }
    }

    pub fn push_back_res(&mut self, result: String) {
        self.res.push(result);
    }

    pub fn push_front_res(&mut self, result: String) {
        if !result.is_empty() && !self.is_only_space(result.clone()) {
            self.res.insert(0, result);
        } else {
            self.res.push(result);
        }
    }

    pub fn transfert(&mut self) {
        for elem in self.tmp_buff.iter() {
            self.res.push(elem.to_string());
        }
        self.tmp_buff.clear();
    }

    pub fn add_to_cache(&mut self, res_line: ResultLine) {
        self.cache.push(res_line);
    }

    fn dump_line_of_len(&mut self, length: usize) {
        let line = (0..length).map(|_| "─").collect::<String>();
        self.push_front_res(line);
    }

    pub fn format_result_cache(&mut self) {
        if self.cache.len() == 0 {
            return;
        }
        let iterator = self.cache.iter();
        let buffer = iterator.map(|&x| { x.1 }).collect::<Vec<isize>>();

        println!("{:?}", buffer[0].to_owned());
        let maxes = fm::get_max_and_index(buffer);
        let mut tmp: String;
        let mut c: char;
        let length = maxes.0.to_string().len();
        let power = fm::get_power_of_two(maxes.0);

        for elem in self.cache.clone() {
            tmp = fm::align_values((elem.0, elem.1), power, length);
            c = tmp.chars().nth(0).unwrap();
            if (c != '=' && c != ' ') || c == '~' {
               self.dump_line_of_len(tmp.len());
            }
            self.push_front_res(tmp)
        }
        self.push_front_res("#".to_string());
        self.cache.clear();

        println!("max {} ---------------------------- index {}", maxes.0, maxes.1);
    }
}

impl Draw for Results {
    fn draw(&self, canvas: &mut dyn Canvas) -> DrawResult<()> {
        let mut row: usize = 1;
        let attr = Attr { fg: Color::LIGHT_WHITE, ..Attr::default() };
        let len = self.res.len();

        for index in 0..len {
            if self.res[index] == "#" {
                row += 1;
                continue;
            }
            if self.res[index].chars().nth(0).unwrap() == '─' || self.res[index].starts_with("Error:") {
                canvas.print_with_attr(row, 0, &self.res[index], attr).unwrap();
            } else {
                canvas.print(row, 0, &self.res[index]).unwrap();
            }
            row+=1;
        }
        Ok(())
    }
}
