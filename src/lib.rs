pub mod cli;
pub mod docs;
pub mod editor;
pub mod import;
pub mod search;
pub mod storage;

pub mod app {
    pub fn fatal(msg: &str) -> ! {
        eprintln!("{msg}");
        std::process::exit(1);
    }
}
