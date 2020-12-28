use tuikit::prelude::*;

// Dummy struct until operaions gets implemented
pub struct Results {
    history: Vec<String>,
}

impl Results {
    pub fn new() -> Self {
        Results { history: Vec::new() }
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
}

impl Draw for Results {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        let mut row: usize = 1;

        self.history
            .iter()
            .rev()
            .for_each(|res| {
                canvas.print(row, 0, &res).unwrap();
                row+=1;
            });
        Ok(())
    }
}
