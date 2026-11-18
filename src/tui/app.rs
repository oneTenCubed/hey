use std::{
    process::exit,
    sync::atomic::{AtomicBool, Ordering},
};

use libc::{EXIT_FAILURE, STDIN_FILENO, read};

use crate::{
    editor, search,
    tui::{term, window},
};

static RESIZED: AtomicBool = AtomicBool::new(false);

pub fn run() {
    if !term::stdout_is_tty() {
        println!("Error! STDOUT is not a terminal!");
        return;
    }

    // Handles termios and raw mode cleanup on exit
    let _guard = term::TerminalGuard::new();

    // Signal handler to handle window resize
    let mut sa: libc::sigaction = unsafe { std::mem::zeroed() };
    sa.sa_sigaction = sigwinch_handler as *const () as libc::sighandler_t;
    unsafe {
        libc::sigaction(libc::SIGWINCH, &sa, std::ptr::null_mut());
    }

    let mut state = State {
        ws: term::get_winsize(),

        cursor_start: 3,
        cursor_end: 3,
        cursor_cur: 3,

        sb_offset: 0,
        sb_row: 1,
        pb_col_start: 0,
        lv_row_start: 3,
        lv_col_start: 1,

        lv_index: 0,

        sb_buf: Vec::new(),
        lv_fields: Vec::new(),
    };

    window::draw(&mut state);

    event_loop(&mut state);
}

pub struct State {
    pub ws: term::Winsize,

    pub cursor_start: u16,
    pub cursor_end: u16,
    pub cursor_cur: u16,

    pub sb_offset: u16,
    pub sb_row: u16,
    pub pb_col_start: u16,
    pub lv_row_start: u16,
    pub lv_col_start: u16,

    pub lv_index: usize,

    pub sb_buf: Vec<u8>,
    pub lv_fields: Vec<search::Field>,
}

fn event_loop(mut state: &mut State) {
    let mut buf = [0u8; 1];
    loop {
        if unsafe { read(STDIN_FILENO, buf.as_mut_ptr().cast(), 1) } == -1 {
            if RESIZED.swap(false, Ordering::Relaxed) {
                state.ws = term::get_winsize();
                window::draw(&mut state);
                continue;
            } else {
                eprintln!("read failed!");
                exit(EXIT_FAILURE);
            }
        }

        match buf[0] {
            // Exit
            0x03 | 0x11 => break,

            // Search input
            0x41..=0x5A | 0x61..=0x7A | 0x30..=0x39 | 0x20 => {
                window::sb_enumerate(buf[0], &mut state)
            }

            // Delete
            0x08 | 0x7F => window::sb_delete(&mut state),

            // Enter
            0x0D | 0x0A => {
                if state.lv_fields.len() != 0 {
                    let field = &state.lv_fields[state.lv_index];
                    editor::open_editor(field.file.clone());
                    break;
                }
            }

            // ESC key
            0x1B => {
                unsafe {
                    read(STDIN_FILENO, buf.as_mut_ptr().cast(), 1);
                }

                match buf[0] {
                    // [ key
                    0x5B => {
                        unsafe {
                            read(STDIN_FILENO, buf.as_mut_ptr().cast(), 1);
                        }

                        match buf[0] {
                            // Arrow keys
                            0x41..=0x44 | 0x46 | 0x48 => window::sb_move_cursor(buf[0], &mut state),
                            _ => {}
                        }
                    }
                    // 0 key
                    0x4F => {}
                    _ => break,
                }
            }

            _ => {}
        }

        let new_fields = search::search(String::from_utf8_lossy_owned(state.sb_buf.clone()));
        if new_fields != state.lv_fields {
            state.lv_fields = new_fields;
            state.lv_index = 0;
            window::lv_draw(&state);
            window::pb_enumerate(&state);
        }
    }
}

extern "C" fn sigwinch_handler(_: libc::c_int) {
    RESIZED.store(true, Ordering::Relaxed);
}
