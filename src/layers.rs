use tuikit::key::Key;
use tuikit::prelude::*;

use super::binary_operation::OperationInterpreter;
use super::query::Query;

pub trait DispLayer {
    // fn display(&self);
    fn run(&mut self, boi: &mut OperationInterpreter);
}

pub struct TuiLayer {
    term: Term<()>,
    query: Query,
}

impl TuiLayer {
    pub fn new() -> Self {
        let _term: Term<()> = Term::with_options(
            TermOptions::default()
                .height(TermHeight::Percent(30))
                .mouse_enabled(true),
        )
        .unwrap();
        TuiLayer {
            term: _term,
            query: Query::new(),
        }
    }
}

impl DispLayer for TuiLayer {
    fn run(&mut self, boi: &mut OperationInterpreter) {
        self.term.present().unwrap();

        while let Ok(ev) = self.term.poll_event() {
            let _ = self.term.clear();

            match ev {
                Event::Key(Key::ESC) | Event::Key(Key::Ctrl('c')) => break,
                // Event::Key(Key::SingleClick(MouseButton::Left, _col, _row)) => {
                //     check_bin_cells(
                //         &mut v_lines,
                //         Rectangle {
                //             top: _col as usize,
                //             left: _row as usize,
                //             width: 1,
                //             height: 1,
                //         },
                //     );
                // }
                Event::Key(Key::Char(ch)) => {
                    self.query.add_char_to_input(ch);
                }
                Event::Key(Key::Backspace) => self.query.rm_char_to_input(),
                Event::Key(Key::Enter) => {
                    let res = self.query.get_input();
                    boi.lexer(&res);
                    match boi.parser() {
                        Ok(_) => boi.interpreter(),
                        Err(err) => {
                            boi.result.push_front_res(err);
                            boi.result.push_front_res("#".to_string());
                        }
                    };
                }
                _ => {}
            }
            self.term.draw(&self.query).unwrap();
            self.term.draw(&boi.result).unwrap();
            // v_lines.iter().for_each(|line| term.draw(line).unwrap());
            let _ = self.term.present();
        }
    }
}

struct StdoLayer {
    query: String,
}

impl StdoLayer {
    pub fn new() -> Self {
        StdoLayer { query: String::new() }
    }
}

impl DispLayer for StdoLayer {
    fn run(&mut self, boi: &mut OperationInterpreter) {
        boi.lexer(&self.query);
        boi.parser().unwrap(); // TODO: check errs
        boi.interpreter();

        for res in boi.result.res.iter() {
            println!("{}", res);
        }
    }
}
