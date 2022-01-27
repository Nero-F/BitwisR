use std::sync::atomic::{ AtomicUsize, Ordering };
use tuikit::prelude::*;

static ID: AtomicUsize = AtomicUsize::new(0); 
pub const COL_BEG: usize = 100;
pub const ROW_BEG: usize = 3;

#[allow(dead_code)]
pub struct BitsLine {
    pub dec_value: String,
    pub bin_value: String,
    pub hex_value: String,
    pub value: isize,
    pub zone: Rectangle
}

impl Draw for BitsLine {
    fn draw(&self, canvas: &mut dyn Canvas) -> DrawResult<()> {
        let row = self.zone.top;
        let mut col = self.zone.left;
        let mut attr = Attr { fg: Color::LIGHT_WHITE, effect: Effect::BOLD, ..Attr::default() };
        let mut v = format!("{} ", self.value);
        
        canvas.print_with_attr(row, col - v.len(), &v, attr).unwrap();
        for letter in self.bin_value.as_str().chars() {
            if letter == '1' {
                attr.fg = Color::LIGHT_WHITE;
            } else {
                attr.fg = Color::Default;
            }
            canvas.put_char_with_attr(row, col, letter, attr).unwrap();
            col += 1;
        }
        v = format!(" {:#x}", self.value);
        attr.effect = Effect::DIM;
        canvas.print_with_attr(row, col, &v, attr).unwrap();
        Ok(())
    }
}

#[allow(dead_code)]
impl BitsLine {
    pub fn new(value: isize) -> BitsLine {
        let id = ID.fetch_add(1, Ordering::SeqCst);

        BitsLine {
            dec_value: value.to_string(),
            bin_value: format!("{:08b}", value),
            hex_value: format!("{:x}", value),
            value: value,
            zone: Rectangle { top: ROW_BEG + id, left: COL_BEG, width: 8, height: 1 }
        }
    }

    pub fn update_values(&mut self, new_val: isize) {
        self.value = new_val;
        self.bin_value = format!("{:08b}", new_val);
        self.hex_value = format!("{:x}", new_val);
    }

    fn update_value_from_binary(&mut self) {
        self.value = isize::from_str_radix(&self.bin_value, 2).unwrap();
    }

    pub fn update_bin_value(&mut self, index: usize) {
        let changed = if self.bin_value.chars().nth(index).unwrap() == '0' {1} else {0};

        self.bin_value.replace_range(index..index+1, &changed.to_string());
        self.update_value_from_binary();
    }
}
