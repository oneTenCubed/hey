use crate::tui::{
    ansi::{self, move_to},
    app,
};

use std::{
    fs::File,
    io::{BufRead, BufReader, Write, stdout},
};

// sb = search bar
// pb = preview box
// lv = list view

pub fn draw(state: &mut app::State) {
    ansi::cursor_hide();

    state.pb_col_start = state.ws.col / 2 + 1;
    state.cursor_end = std::cmp::min(state.cursor_end, state.pb_col_start - 2);
    state.cursor_cur = state.cursor_end;
    pb_draw(&state);
    pb_enumerate(&state);

    sb_draw(&state);

    lv_draw(&state);

    move_to(state.sb_row, state.cursor_cur);
    ansi::cursor_show();
}

// Preview Box
fn pb_draw(state: &app::State) {
    move_to(1, 1);
    for i in 1..=state.ws.row {
        if i == 1 || i == state.ws.row {
            print!("{:1$}", "", state.pb_col_start as usize);
            for j in state.pb_col_start..state.ws.col {
                if j == state.pb_col_start {
                    if i == 1 {
                        print!("┌"); // ╭
                    } else {
                        print!("└"); // ╰
                    }
                } else if j == state.ws.col - 1 {
                    if i == 1 {
                        print!("┐\n"); // ╮
                    } else {
                        print!("┘"); // ╯
                    }
                } else {
                    print!("─");
                }
            }
            stdout().flush().unwrap();
        } else {
            print!("{:1$}│", "", state.pb_col_start as usize);
            print!(
                "{:1$}│\n",
                "",
                (state.ws.col - state.pb_col_start) as usize - 2
            );
            stdout().flush().unwrap();
        }
    }
}

pub fn pb_enumerate(state: &app::State) {
    if !state.lv_fields.is_empty() {
        let width = (state.ws.col - state.pb_col_start - 2) as usize;

        let mut line = String::new();
        let field = &state.lv_fields[state.lv_index];
        let read = File::open(field.file.clone());
        let mut reader;
        match read {
            Ok(file) => {
                reader = BufReader::new(file);
            }
            _ => return,
        }

        ansi::cursor_hide();
        pb_clear(&state);

        let mut i = 2;
        while reader.read_line(&mut line).unwrap() != 0 && i < state.ws.row {
            move_to(i, state.pb_col_start + 2);
            i += 1;
            print!("{}", &line[..std::cmp::min(line.len(), width)]);
            line.clear();
        }
        stdout().flush().unwrap();

        move_to(state.sb_row, state.cursor_cur);
        ansi::cursor_show();
    } else {
        pb_clear(&state);
        move_to(state.sb_row, state.cursor_cur);
    }
}

fn pb_clear(state: &app::State) {
    let width = (state.ws.col - state.pb_col_start - 2) as usize;

    for i in 2..state.ws.row {
        move_to(i, state.pb_col_start + 2);
        println!("{:1$}", "", width as usize);
    }
}

// Search Bar
pub fn sb_enumerate(token: u8, state: &mut app::State) {
    state.sb_buf.insert(
        (state.cursor_cur - state.cursor_start + state.sb_offset) as usize,
        token,
    );
    if state.cursor_cur + 2 < state.pb_col_start {
        state.cursor_cur += 1;
        if state.cursor_end + 2 < state.pb_col_start {
            state.cursor_end += 1;
        }
    } else {
        state.sb_offset += 1;
    }

    sb_draw(&state);
}

fn sb_draw(state: &app::State) {
    ansi::cursor_hide();
    move_to(state.sb_row, state.cursor_start - 2);

    println!(
        "$ {:1$}",
        " ",
        (state.cursor_end - state.cursor_start) as usize + 1
    );
    move_to(state.sb_row, state.cursor_start);

    for c in &state.sb_buf[(state.sb_offset as usize)
        ..((state.cursor_end - state.cursor_start + state.sb_offset) as usize)]
    {
        print!("{}", *c as char);
    }
    stdout().flush().unwrap();

    move_to(state.sb_row + 1, state.cursor_start);
    println!(
        "{:─>1$}",
        "",
        (state.ws.col - state.pb_col_start - 1) as usize
    );

    move_to(state.sb_row, state.cursor_cur);
    ansi::cursor_show();
}

// TODO: current list view doesnt take into account the size available to be drawn, so overflow
// occurs, causing in the entire tui breaking. make changes to handle overflow and list offset
pub fn sb_move_cursor(dir: u8, state: &mut app::State) {
    match dir {
        // Up
        0x41 => {
            if state.lv_index > 0 {
                state.lv_index -= 1;
                lv_update_index(&state);
                pb_enumerate(&state);
            }
        }
        // Down
        0x42 => {
            if (state.lv_index as i16) < (state.lv_fields.len() as i16) - 1 {
                state.lv_index += 1;
                lv_update_index(&state);
                pb_enumerate(&state);
            }
        }
        // Right
        0x43 => {
            if state.cursor_cur + 1 <= state.cursor_end {
                state.cursor_cur += 1;
                move_to(state.sb_row, state.cursor_cur);
            } else if state.cursor_end - state.cursor_start + state.sb_offset
                < state.sb_buf.len() as u16
            {
                state.sb_offset += 1;
                sb_draw(&state);
            }
        }
        // Left
        0x44 => {
            if state.cursor_cur - 1 >= state.cursor_start {
                state.cursor_cur -= 1;
                move_to(state.sb_row, state.cursor_cur);
            } else if state.sb_offset > 0 {
                state.sb_offset -= 1;
                sb_draw(&state);
            }
        }
        // Home
        0x48 => {
            state.sb_offset = 0;
            state.cursor_cur = state.cursor_start;
            state.cursor_end = state.cursor_start
                + std::cmp::min(
                    state.pb_col_start - 2 - state.cursor_start,
                    state.sb_buf.len() as u16,
                );
            move_to(state.sb_row, state.cursor_cur);
            sb_draw(&state);
        }
        // End
        0x46 => {
            state.sb_offset = state.sb_buf.len() as u16
                - (state.cursor_end - state.cursor_start + state.sb_offset);
            state.cursor_cur = state.cursor_end;
            move_to(state.sb_row, state.cursor_cur);
            sb_draw(&state);
        }

        _ => {}
    }
}

pub fn sb_delete(state: &mut app::State) {
    let pos = (state.cursor_cur - state.cursor_start + state.sb_offset) as usize;
    if pos > 0 {
        state.sb_buf.remove(pos - 1);
        if state.cursor_cur > state.cursor_start {
            state.cursor_cur -= 1;
            state.cursor_end -= 1;
        }
        sb_draw(&state);
    }
}

// List View
pub fn lv_draw(state: &app::State) {
    move_to(state.lv_row_start, state.lv_col_start);
    lv_clear(&state);
    move_to(state.lv_row_start, state.lv_col_start);

    let width = (state.ws.col - state.pb_col_start - 1) as usize;
    let fields = state.lv_fields.clone();
    for (i, field) in fields.iter().enumerate() {
        println!(
            "{} {}",
            if i == state.lv_index { ">" } else { " " },
            &field.display_name[..std::cmp::min(field.display_name.len(), width)]
        );
    }

    move_to(state.sb_row, state.cursor_cur);
}

fn lv_clear(state: &app::State) {
    let width = state.ws.col - state.pb_col_start;
    let height = state.ws.row - state.lv_row_start;

    for _ in state.lv_row_start..=height {
        println!("{:1$}", "", width as usize + 1);
    }
}

fn lv_update_index(state: &app::State) {
    move_to(state.lv_row_start, state.lv_col_start);
    for i in 0..state.lv_fields.len() {
        println!("{}", if i == state.lv_index { ">" } else { " " });
    }
    move_to(state.sb_row, state.cursor_cur);
}
