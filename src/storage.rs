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
    } // same as open_editor(), merge and refactor :: TODO!
}

pub fn open_editor(title: String) {
    // TODO!! ^^^ merge and refactor
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

pub fn get_note_titles() -> Vec<String> {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => {
            panic!("Unable to read path to home directory");
        }
    };
    let path_to_notes = home_dir.join(".local/share/hey/notes/");

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

        let binding = entry.path();
        let entry = binding.file_name();
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

// !!!TODO
// home_dir, path_to_file repeated nearly everywhere. refactor and introduce helper functions
