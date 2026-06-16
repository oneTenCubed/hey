use crate::app::fatal;
use dirs;
use std::{
    env,
    fs::{self, DirBuilder},
    io::{self, Write},
    path::PathBuf,
    process,
};

fn get_hey_local_data_dir() -> PathBuf {
    let local_data_dir = dirs::data_local_dir();

    match local_data_dir {
        Some(path) => path.join("hey"),
        None => {
            fatal("Unable to find local data directory!");
        }
    }
}

pub fn get_hey_notes_dir() -> PathBuf {
    let notes_dir = get_hey_local_data_dir().join("notes");

    if notes_dir.is_dir() {
        notes_dir
    } else {
        fatal("Unable to find notes data directory!");
    }
}

pub fn initialize_storage() {
    let root_dir_path = get_hey_local_data_dir().join("hey");
    let root_dir_stat = DirBuilder::new().recursive(true).create(&root_dir_path);
    match root_dir_stat {
        Ok(_) => (),
        Err(_) => {
            fatal("Error initialising hey dir"); // TODO: add cli error. i.e, match error type
        }
    }

    let notes_dir_path = root_dir_path.join("notes");
    let notes_dir_stat = DirBuilder::new().recursive(true).create(&notes_dir_path);
    match notes_dir_stat {
        Ok(_) => (),
        Err(_) => {
            fatal("Error initialising notes dir"); // TODO ^^^^
        }
    }
}

pub fn new_article(title: String) {
    let path_to_file = get_hey_notes_dir().join(&title);

    let mut open_editor_flag = true;
    if path_to_file.is_file() {
        let mut input = String::new();

        print!(":: File with matching title already exists, open it instead? [Y/n] ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Error reading!");

        open_editor_flag = match &(input.trim())[..] {
            "n" => false,
            "N" => false,
            _ => true,
        }
    }

    if open_editor_flag {
        open_editor(title);
    }
}

// TODO: make an editor module
pub fn open_editor(title: String) {
    let path_to_file = get_hey_notes_dir().join(title);

    let editor = env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string());

    let _cmnd = process::Command::new(editor)
        .arg(path_to_file)
        .status()
        .expect("Couldn't open editor.");
}

pub fn get_note_titles() -> Vec<String> {
    let path_to_notes = get_hey_notes_dir();

    let mut titles: Vec<String> = Vec::new();

    for entry in match fs::read_dir(&path_to_notes) {
        Ok(entry) => entry,
        _ => {
            println!("Unexpected error occurred!");
            // TODO: handle errors such as permission denied or broken symlinks
            return Vec::new();
        }
    } {
        let entry = match entry {
            Ok(entry) => entry,
            _ => {
                println!("Unexpected error occurred!");
                return Vec::new();
            }
        };

        let entry = entry.path();
        let entry = entry.file_name();
        let entry = match entry {
            Some(entry) => match entry.to_str() {
                Some(entry) => entry,
                _ => continue,
            },
            _ => continue,
        };

        titles.push(entry.to_string());
    }

    titles
}

pub fn read_article(title: String) {
    let path_to_file = get_hey_notes_dir().join(title);

    let file_content = fs::read_to_string(path_to_file);
    match file_content {
        Ok(s) => {
            println!("\n{}", s);
        }
        Err(_) => {
            eprintln!("Couldn't read file contents");
        }
    }
}
