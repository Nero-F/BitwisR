#[path = "bitsline.rs"]
mod bl;

use std::cmp::{min, max};
use tuikit::prelude::*;

fn main() {
    let term: Term<()> = Term::with_options(TermOptions::default()
                                                .height(TermHeight::Percent(30))
                                                .mouse_enabled(true))
                            .unwrap();
    let mut row = 1;
    let mut col = 0;

    let _ = term.present();
    let mut v_lines: [bl::BitsLine; 8] = [
        bl::BitsLine::new(1),
        bl::BitsLine::new(2),
        bl::BitsLine::new(4),
        bl::BitsLine::new(8),
        bl::BitsLine::new(16),
        bl::BitsLine::new(32),
        bl::BitsLine::new(64),
        bl::BitsLine::new(128)
    ];

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();

        let (width, height) = term.term_size().unwrap();
        match ev {
            Event::Key(Key::ESC) | Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Up) => row = max(row-1, 1),
            Event::Key(Key::Down) => row = min(row+1, height-1),
            Event::Key(Key::Left) => col = max(col, 1)-1,
            Event::Key(Key::Right) => col = min(col+1, width-1),
            Event::Key(Key::SingleClick(MouseButton::Left, _col, _row)) => {
                check_bin_cells(&mut v_lines, Rectangle {top: _col as usize, left: _row as usize, width: 1, height: 1 });
            }
            _ => {}
        }
        v_lines
            .iter()
            .for_each(|line| term.draw(line).unwrap());
        let _ = term.set_cursor(row, col);
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
