use crate::storage;
use std::{env, process};

fn get_editor() -> String {
    let default: String;

    if cfg!(target_os = "windows") {
        default = "notepad".to_string();
    } else {
        default = "vi".to_string();
    }

    env::var("HEY_EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or(default)
}

pub fn open_editor(title: String) {
    let path_to_file = storage::get_hey_notes_dir().join(title);

    let editor = get_editor();

    let _cmnd = match process::Command::new(editor).arg(path_to_file).status() {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Couldn't open editor!");
        }
    };
}
