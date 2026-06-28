use crate::{app::fatal, editor};
use dirs;
use std::{
    fs::{self, DirBuilder},
    io::{self, Read, Write},
    path::PathBuf,
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

pub fn get_hey_dir(title: &str) -> PathBuf {
    let directory = get_hey_local_data_dir().join(title);

    if directory.is_dir() {
        directory
    } else {
        fatal(format!("Unable to find {} data directory!", title).as_str());
    }
}

pub fn initialize_storage() {
    let root_dir_path = get_hey_local_data_dir();
    let root_dir_stat = DirBuilder::new().recursive(true).create(&root_dir_path);
    match root_dir_stat {
        Ok(_) => (),
        Err(_) => {
            fatal("Error initialising hey dir!"); // TODO: add cli error. i.e, match error type
        }
    }

    let notes_dir_path = root_dir_path.join("notes");
    let notes_dir_stat = DirBuilder::new().recursive(true).create(&notes_dir_path);
    match notes_dir_stat {
        Ok(_) => (),
        Err(_) => {
            fatal("Error initialising notes dir!"); // TODO ^^^^
        }
    }

    let imports_dir_path = root_dir_path.join("imports");
    let imports_dir_stat = DirBuilder::new().recursive(true).create(&imports_dir_path);
    match imports_dir_stat {
        Ok(_) => (),
        Err(_) => {
            fatal("Error initialising imports dir!"); // TODO ^^^^
        }
    }
}

pub fn new_article(title: String) {
    let mut title: Vec<&str> = title.split(' ').collect();
    title.sort();
    let mut title: String = title.join("_");
    title.push_str(".txt"); // TODO: also support md on request

    let path_to_file = get_hey_dir("notes").join(&title);

    let mut open_editor_flag = true;
    if path_to_file.is_file() {
        let mut input = String::new();

        print!(":: File with matching title already exists, open it instead [Yn]? ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Error reading!");

        open_editor_flag = match &(input.trim())[..] {
            "n" | "N" | "q" | "Q" => false,
            _ => true,
        }
    }

    if open_editor_flag {
        editor::open_editor(get_hey_dir("notes").join(title));
    }
}

pub fn get_note_titles(path_to_notes: PathBuf) -> Vec<PathBuf> {
    let mut titles: Vec<PathBuf> = Vec::new();

    for entry in match fs::read_dir(&path_to_notes) {
        Ok(entry) => entry,
        _ => {
            eprintln!("Unexpected error occurred!");
            // TODO: handle errors such as permission denied or broken symlinks
            return Vec::new();
        }
    } {
        let entry = match entry {
            Ok(entry) => entry,
            _ => {
                eprintln!("Unexpected error occurred!");
                return Vec::new();
            }
        };

        titles.push(entry.path());
        /*let entry = entry.path();
        let entry = entry.file_name();
        let entry = match entry {
            Some(entry) => match entry.to_str() {
                Some(entry) => entry,
                _ => continue,
            },
            _ => continue,
        };

        titles.push(entry.to_string());*/
    }

    titles
}

pub fn enumerate_kw_editor_file(path_to_file: PathBuf, data: Vec<&str>) -> bool {
    let mut file = match fs::File::create(path_to_file) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Unable to enumerate editor file with keywords!");
            return false;
        }
    };

    for keyword in data {
        let _ = file.write_fmt(format_args!("{}\n", keyword));
    }

    true
}

pub fn read_kw_editor_file(path_to_file: PathBuf) -> Vec<String> {
    let mut file = match fs::OpenOptions::new().read(true).open(path_to_file) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Unable to read editor file with keywords!");
            return Vec::new();
        }
    };

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let mut keywords = Vec::new();
    for kw in data.trim().split("\n") {
        keywords.push(kw.to_string());
    }
    keywords.sort();

    keywords
}

pub fn copy_file(old_location: PathBuf, new_location: PathBuf) {
    // TODO: handle errors like disk full, permission denied, ro destination etc...
    let status = fs::copy(old_location, new_location);

    match status {
        Ok(_) => (),
        Err(err) => fatal(format!("Error occurred: {}", err).as_str()),
    }
}
