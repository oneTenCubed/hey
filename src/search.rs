use crate::cli;
use std::{env, process};

#[derive(Debug, Clone)]
pub struct Field {
    pub matches: i32,
    pub file: String,
}

pub fn search(s: String) {
    let binding = s.to_lowercase();
    let search_tokens: Vec<&str> = binding.split(' ').collect();

    let search_field = get_note_titles();
    let search_field = match String::from_utf8(search_field) {
        Ok(val) => val.trim().to_string(),
        _ => String::new(),
    };

    let titles: Vec<&str> = search_field.split("\n").collect();
    let mut result: Vec<Field> = Vec::new();

    for title in titles {
        let title_tokens: Vec<&str> = title.split(' ').collect();
        let mut count = 0;

        for title_token in &title_tokens {
            for search_token in &search_tokens {
                if title_token == search_token {
                    count += 1;
                }
            }
        }

        result.push(Field {
            matches: count,
            file: title.to_string(),
        });
    }

    let n = result.len();
    let mut temp;
    for i in 0..(n - 1) {
        for j in 0..(n - i - 1) {
            if result[j].matches < result[j + 1].matches {
                temp = result[j].clone();
                result[j] = result[j + 1].clone();
                result[j + 1] = temp;
            }
        }
    }

    let mut search_results = Vec::new();
    for item in result {
        if item.matches != 0 {
            search_results.push(item);
        } else {
            break;
        }
    }

    cli::search_result(search_results);
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
