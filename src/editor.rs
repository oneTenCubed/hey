use crate::storage;
use std::{env, process};

fn get_editor() -> String {
    let default: String;

    if cfg!(target_os = "windows") {
        default = "Notepad".to_string();
    } else {
        default = "vi".to_string();
    }

    env::var("HEY_EDITOR") // TODO: mention in docs
        .or_else(|_| env::var("VISUAL"))
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or(default)
}

pub fn open_editor(title: String) {
    let path_to_file = storage::get_hey_notes_dir().join(title);

    let editor = get_editor();

    let _cmnd = process::Command::new(editor)
        .arg(path_to_file)
        .status()
        .expect("Couldn't open editor.");
}
