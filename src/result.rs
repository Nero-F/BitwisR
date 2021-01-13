use tuikit::prelude::*;

// Dummy struct until operaions gets implemented
pub struct Results {
    history: Vec<String>,
    pub res: Vec<String>,
}

impl Results {
    pub fn new() -> Self {
        Results { history: Vec::new(), res: Vec::new() }
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
}

impl Draw for Results {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
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
