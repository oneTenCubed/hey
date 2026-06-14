use std::{
    env,
    fs::{self, DirBuilder},
    io::{self, Write},
    path::Path,
    process,
};

pub fn initialize_storage() {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            panic!("Unable to read path to home directory");
        }
    };

    let root_dir_path = format!("{}/.local/share/hey", home_dir.display());
    let root_dir_stat = DirBuilder::new().recursive(true).create(&root_dir_path);
    match root_dir_stat {
        Ok(_) => (),
        Err(_) => {
            panic!("Error initialising base directory: {}", root_dir_path);
        }
    }

    let notes_dir_path = format!("{}/.local/share/hey/notes", home_dir.display());
    let notes_dir_stat = DirBuilder::new().recursive(true).create(&notes_dir_path);
    match notes_dir_stat {
        Ok(_) => (),
        Err(_) => {
            panic!("Error initialising notes directory: {}", notes_dir_path);
        }
    }
}

pub fn new_article(title: String) {
    let editor = env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string());

    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            panic!("Unable to read path to home directory");
        }
    };
    let path_to_file = format!("{}/.local/share/hey/notes/{}", home_dir.display(), title);

    let mut open_editor_flag = true;
    if Path::new(&path_to_file).is_file() {
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
        let _cmnd = process::Command::new(editor)
            .arg(path_to_file)
            .status()
            .expect("Couldn't open editor.");
    }
}

pub fn open_editor(title: String) {
    let editor = env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string());

    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            panic!("Unable to read path to home directory");
        }
    };
    let path_to_file = format!("{}/.local/share/hey/notes/{}", home_dir.display(), title);

    let _cmnd = process::Command::new(editor)
        .arg(path_to_file)
        .status()
        .expect("Couldn't open editor.");
}

pub fn get_note_titles() -> Vec<u8> {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            panic!("Unable to read path to home directory");
        }
    };
    let path_to_notes = format!("{}/.local/share/hey/notes/", home_dir.display());

    let cmnd = process::Command::new("ls")
        .arg(path_to_notes)
        .output()
        .expect("Couldn't find notes directory");

    cmnd.stdout
}

pub fn cat_article(title: String) {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            panic!("Unable to read path to home directory");
        }
    };
    let path_to_file = format!("{}/.local/share/hey/notes/{}", home_dir.display(), title);

    let file_content = fs::read_to_string(path_to_file);
    match file_content {
        Ok(s) => {
            println!("\n{}", s);
        }
        Err(_) => {
            println!("Couldn't read file contents");
        }
    }
}
