use std::{collections::VecDeque, vec};

use super::fm;
use tuikit::prelude::*;

#[derive(Clone)]
pub enum Line {
    Number((char, isize)),
    Under,
    Null,
}

type ResultLine = (char, isize);

#[allow(dead_code)]
pub struct Results {
    history: Vec<String>,
    pub res: Vec<String>,
    pub tmp_buff: Vec<String>,
    pub cache: Vec<ResultLine>,
    pub cache_x: VecDeque<Line>,
}

impl Results {
    pub fn new() -> Self {
        Results {
            history: Vec::new(),
            res: Vec::new(),
            tmp_buff: Vec::new(),
            cache: Vec::new(),
            cache_x: VecDeque::new(),
        }
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
    pub fn push_front_res(&mut self, result: String) {
        if !result.is_empty() && !self.is_only_space(result.clone()) {
            self.res.insert(0, result);
        } else {
            self.res.push(result);
        }
    }

    fn dump_line_of_len(&mut self, length: usize) {
        let line = (0..length).map(|_| "─").collect::<String>();
        self.push_front_res(line);
    }

    //this iterata over all the nums to find the biggest and allign each vals
    fn get_max_of_cached_values(&self) -> Option<isize> {
        let mut buff = vec![];

        for x in self.cache_x.clone() {
            if let Line::Number((_, n)) = x {
                buff.push(n);
            }
        }
        buff.iter().max().copied()
    }

    // Todo: finish this
    pub fn format_result_cached(&mut self) {
        let max = if let Some(x) = self.get_max_of_cached_values() {
            x
        } else {
            return;
        };
        let l = if max < 0 { 0 } else { max.to_string().len() };
        let mut width = 0;

        for elem in self.cache_x.clone() {
            match elem {
                Line::Number(x) => {
                    let aligned = fm::align_values(x, l);
                    width = aligned.len();
                    self.push_front_res(aligned);
                }
                Line::Under => self.dump_line_of_len(width),
                Line::Null => {}
            }
        }
        self.push_front_res("#".to_string());
        self.cache_x.clear();
    }
}

impl Draw for Results {
    fn draw(&self, canvas: &mut dyn Canvas) -> DrawResult<()> {
        let mut row: usize = 1;
        let attr = Attr {
            fg: Color::LIGHT_WHITE,
            ..Attr::default()
        };
        let len = self.res.len();

        for index in 0..len {
            if self.res[index] == "#" {
                row += 1;
                continue;
            }
            if self.res[index].chars().nth(0).unwrap() == '─'
                || self.res[index].starts_with("Error:")
            {
                canvas
                    .print_with_attr(row, 0, &self.res[index], attr)
                    .unwrap();
            } else {
                canvas.print(row, 0, &self.res[index]).unwrap();
            }
            row += 1;
        }
        Ok(())
    }
}
