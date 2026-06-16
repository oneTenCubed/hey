pub mod cli;
pub mod search;
pub mod storage;

pub mod app {
    pub fn fatal(msg: &str) -> ! {
        eprintln!("{msg}");
        std::process::exit(1);
    }
}
