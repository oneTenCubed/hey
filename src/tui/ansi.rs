use std::io::{Write, stdout};

const ESC: &str = "\x1B";

// Both the row and col fields are 1-indexed
pub fn move_to(row: u16, col: u16) {
    print!("{ESC}[{row};{col}H");
    stdout().flush().unwrap();
}

pub fn altscreen_start() {
    print!("{}[?1049h", ESC);
    stdout().flush().unwrap();
}

pub fn altscreen_end() {
    print!("{}[?1049l", ESC);
    stdout().flush().unwrap();
}

pub fn cursor_hide() {
    print!("{}[?25l", ESC);
    stdout().flush().unwrap();
}

pub fn cursor_show() {
    print!("{}[?25h", ESC);
    stdout().flush().unwrap();
}
