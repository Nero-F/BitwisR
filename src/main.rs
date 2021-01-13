#[path = "bitsline.rs"]
mod bl;
mod query;
mod result;
#[path = "binary_operation.rs"]
mod boi;

use tuikit::prelude::*;
use tuikit::key::Key;

fn init_bit_table() -> [bl::BitsLine; 8] {
    let v_lines: [bl::BitsLine; 8] = [
        bl::BitsLine::new(1),
        bl::BitsLine::new(2),
        bl::BitsLine::new(4),
        bl::BitsLine::new(8),
        bl::BitsLine::new(16),
        bl::BitsLine::new(32),
        bl::BitsLine::new(64),
        bl::BitsLine::new(128)
    ];
    v_lines
}

fn main() {
    let term: Term<()> = Term::with_options(TermOptions::default()
                                                .height(TermHeight::Percent(30))
                                                .mouse_enabled(true))
                            .unwrap();
    let _ = term.present();
    let mut query = query::Query::new();
    let mut interpreter = boi::OperationInterpreter::new();
    let mut v_lines = init_bit_table();

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();

        match ev {
            Event::Key(Key::ESC) | Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::SingleClick(MouseButton::Left, _col, _row)) => {
                check_bin_cells(&mut v_lines, Rectangle {top: _col as usize, left: _row as usize, width: 1, height: 1 });
            },
            Event::Key(Key::Char(ch)) => {
                query.add_char_to_input(ch);
            },
            Event::Key(Key::Backspace) => query.rm_char_to_input(),
            Event::Key(Key::Enter) => {
                let res = query.get_input();
                interpreter.lexer(&res);
                match interpreter.parser() {
                    Ok(_) => interpreter.interpreter(),
                    Err(_) => continue
                };
            },
            _ => {}
        }
        term.draw(&query).unwrap();
        term.draw(&interpreter.result).unwrap();
        v_lines
            .iter()
            .for_each(|line| term.draw(line).unwrap());
        let _ = term.present();
    }
}

fn check_bin_cells(lines: &mut [bl::BitsLine; 8], mouse: Rectangle) {
   lines
       .iter_mut()
       .for_each(|line| {
           if line.zone.contains(mouse.top, mouse.left) {
                line.update_bin_value(mouse.left - bl::COL_BEG);
           }
       });
}
