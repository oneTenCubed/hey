use crate::{app::fatal, storage};
use std::{
    collections::HashSet,
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

pub fn import(levels: u8, ignore_tokens: HashSet<&str>, confirm: bool, overwrite: bool) {
    let src_dir: PathBuf = match env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            fatal("Unable to calculate the current working directory!");
        }
    };
    let imports_dir: PathBuf = storage::get_hey_imports_dir();

    let tokens = tokenize(src_dir.clone(), levels, ignore_tokens);
    let titles: Vec<String> = storage::get_note_titles(src_dir.clone());
    let old_path = src_dir.clone();
    let new_path = imports_dir.clone();
    let mut input = String::new();
    let mut counter = 0;

    for title in titles {
        let old_location = old_path.join(&title);
        if !old_location.is_file() || title.chars().nth(0) == Some('.') {
            continue;
        }

        match PathBuf::from(&title).extension() {
            Some(extension) => match extension.to_str() {
                Some(string) => match string {
                    "txt" | "md" => (),
                    _ => continue,
                },
                None => continue,
            },
            None => (),
        }

        let new_title = get_new_title(tokens.clone(), title.clone());
        let new_location = new_path.join(&new_title);

        let keywords: Vec<&str> = new_title.split('.').collect();
        let keywords: Vec<&str> = keywords[0].split('_').collect();
        let file_already_exist = if new_location.is_file() { true } else { false };
        if confirm {
            println!("\n  Importing file \"{}\" with keywords:", &title);
            print!("\t");
            io::stdout().flush().unwrap();
            for keyword in keywords {
                print!("\"{}\" ", keyword);
            }
            io::stdout().flush().unwrap();

            print!(
                "{}\n:: {} file (y) OR edit keywords (e) [Yen]? ",
                if !overwrite && file_already_exist {
                    "\n  File with matching keywords already exist!"
                } else {
                    ""
                },
                if !overwrite && file_already_exist {
                    "Overwrite"
                } else {
                    "Import"
                }
            );
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).expect("  Error reading!");
        } else {
            if overwrite && file_already_exist {
                println!("\n  File with matching keywords already exist!");
                print!("    Keywords: ");
                io::stdout().flush().unwrap();
                for keyword in keywords {
                    print!("\"{}\" ", keyword);
                }
                io::stdout().flush().unwrap();

                print!("\n:: Overwrite file (y) OR edit keywords (e) [Yen]? ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).expect("  Error reading!");
            } else {
                input = String::from("y");
            }
        }

        match input.trim() {
            "q" => {
                break;
            }
            "e" => todo!("keyword editor"), // TODO: implement confirm mode / edit keywords
            "n" => {
                println!("Skipping file...");
                continue;
            }
            _ => {
                let _ = fs::copy(old_location, new_location).expect("  Error copying!");
                // TODO: handle errors like disk full, permission denied, ro destination etc...
                counter += 1;
            }
        }
    }

    println!(
        "{}Imported {} file{}",
        if confirm { "\n" } else { "" },
        counter,
        if counter == 1 { "" } else { "s" }
    );
}

fn tokenize(src_dir: PathBuf, levels: u8, ignore_tokens: HashSet<&str>) -> HashSet<String> {
    let mut tokens: HashSet<String> = HashSet::new();
    let mut file_tokens: Vec<&std::path::Path> = src_dir.ancestors().collect();
    file_tokens.pop();

    for (index, dir) in file_tokens.iter().enumerate() {
        if index as u8 >= levels {
            break;
        }

        let dir_name = dir.file_name();
        let token = match dir_name {
            Some(value) => match value.to_str() {
                Some(value) => value,
                None => {
                    fatal("Unexpected error occurred!");
                }
            },
            None => {
                fatal("Unexpected error occurred!");
            }
        };

        if let None = ignore_tokens.get(&token) {
            let token_split: Vec<&str> = token.split('_').collect();
            for token in token_split {
                if let None = ignore_tokens.get(&token) {
                    tokens.insert(token.to_lowercase().to_string());
                }
            }
        }
    }

    tokens
}

fn get_new_title(mut curr_tokens: HashSet<String>, title: String) -> String {
    let mut new_title = String::new();

    let title_tokens: Vec<&str> = title.split('.').collect();
    let title_tokens: Vec<&str> = title_tokens[0].split('_').collect();
    for token in &title_tokens {
        curr_tokens.insert(token.to_lowercase().to_string());
    }

    let exhaustable_tokens = curr_tokens.clone();
    let mut new_title_tokens: Vec<String> = exhaustable_tokens.into_iter().collect();
    new_title_tokens.sort();
    let new_title_tokens = new_title_tokens.join("_");
    new_title.push_str(new_title_tokens.as_str());

    let file = PathBuf::from(&title);
    match file.extension() {
        Some(extension) => match extension.to_str() {
            Some(string) => {
                new_title.push('.');
                new_title.push_str(string);
            }
            None => {
                fatal("Invalid extension!");
            }
        },
        None => (),
    };

    new_title
}
