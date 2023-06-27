use tuikit::prelude::*;

#[allow(dead_code)]
pub struct Query {
    history: Vec<String>,
    prompt: String,
    query: Vec<char>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            history: Vec::new(),
            prompt: "c> ".to_string(),
            query: Vec::new(),
        }
    }

    pub fn add_char_to_input(&mut self, ch: char) {
        self.query.push(ch);
    }

    pub fn rm_char_to_input(&mut self) {
        self.query.pop();
    }

    pub fn get_input(&mut self) -> String {
        self.query
            .drain(..)
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>()
    }
}

impl Draw for Query {
    fn draw(&self, canvas: &mut dyn Canvas) -> DrawResult<()> {
        canvas.clear()?;
        let prompt = &self.prompt;
        let mut pos = prompt.len();
        let query: String = self.query.iter().collect();

        canvas.print(0, 0, &prompt).unwrap();
        canvas.print(0, pos, &query).unwrap();
        pos += query.len();
        canvas.set_cursor(0, pos).unwrap();
        Ok(())
    }
}
