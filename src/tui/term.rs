use std::{mem::MaybeUninit, process::exit};

use libc::{
    ECHO, EXIT_FAILURE, ICANON, ICRNL, ISIG, IXON, STDOUT_FILENO, TCSANOW, TIOCGWINSZ, VMIN, VTIME,
    ioctl, isatty, tcgetattr, tcsetattr, termios, winsize,
};

use crate::tui::{ansi, term};

#[derive(Debug)]
pub struct Winsize {
    pub row: u16,
    pub col: u16,
}

pub fn get_winsize() -> Winsize {
    let mut sz = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    unsafe {
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut sz);
    }

    Winsize {
        row: sz.ws_row,
        col: sz.ws_col,
    }
}

pub fn stdout_is_tty() -> bool {
    unsafe { matches!(isatty(STDOUT_FILENO), 1) }
}

fn to_raw_mode(tios: &mut termios) {
    tios.c_lflag &= !(ICANON | ECHO | ISIG);
    tios.c_iflag &= !(ICRNL | IXON);
    tios.c_cc[VMIN] = 1;
    tios.c_cc[VTIME] = 0;
}

fn get_termios() -> termios {
    let mut tios = MaybeUninit::<termios>::uninit();

    if unsafe { tcgetattr(STDOUT_FILENO, tios.as_mut_ptr()) } == -1 {
        eprintln!("tcgetattr failed: {}", std::io::Error::last_os_error());
        exit(EXIT_FAILURE);
    }

    unsafe { tios.assume_init() }
}

fn set_termios(tios: &termios) {
    if unsafe { tcsetattr(STDOUT_FILENO, TCSANOW, tios) } == -1 {
        eprintln!("tcsetattr failed: {}", std::io::Error::last_os_error());
        exit(EXIT_FAILURE);
    };
}

pub struct TerminalGuard {
    original: termios,
}

impl TerminalGuard {
    pub fn new() -> Self {
        let tios = term::get_termios();
        let mut new_tios = tios;
        term::to_raw_mode(&mut new_tios);
        term::set_termios(&new_tios);

        ansi::altscreen_start();

        Self { original: tios }
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        ansi::cursor_show();
        ansi::altscreen_end();

        term::set_termios(&self.original);
    }
}
