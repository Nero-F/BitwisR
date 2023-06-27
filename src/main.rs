mod binary_operation;
use binary_operation as boi;
mod bitsline;
use bitsline as bl;
mod formatter;
use formatter as fm;

mod layers;
use layers::{ TuiLayer, DispLayer };

mod query;
mod result;
mod tokens;

// use tuikit::key::Key;
use tuikit::prelude::*;

fn init_bit_table() -> [bl::BitsLine; 8] {
    let v_lines: [bl::BitsLine; 8] = [
        bl::BitsLine::new(1),
        bl::BitsLine::new(2),
        bl::BitsLine::new(4),
        bl::BitsLine::new(8),
        bl::BitsLine::new(16),
        bl::BitsLine::new(32),
        bl::BitsLine::new(64),
        bl::BitsLine::new(128),
    ];
    v_lines
}

fn main() {
    let mut layer = TuiLayer::new();
    let mut interpreter = boi::OperationInterpreter::new();
    layer.run(&mut interpreter);
}

fn check_bin_cells(lines: &mut [bl::BitsLine; 8], mouse: Rectangle) {
    lines.iter_mut().for_each(|line| {
        if line.zone.contains(mouse.top, mouse.left) {
            line.update_bin_value(mouse.left - bl::COL_BEG);
        }
    });
}
