use crate::{app::fatal, cli, editor, storage};
use std::{collections::HashSet, env, path::PathBuf};

// TODO: resolve duplicate file names over notes and imports
// maybe specify whether imported or not
// make it into a feature
pub fn import(
    levels: u8,
    ignore_tokens: HashSet<String>,
    add_tokens: HashSet<String>,
    confirm: bool,
    overwrite: bool,
    files: HashSet<&str>,
    add_before_ignore: bool,
) {
    let src_dir: PathBuf = match env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            fatal("Unable to calculate the current working directory!");
        }
    };
    let imports_dir: PathBuf = storage::get_hey_dir("imports");

    let tokens = tokenize(
        src_dir.clone(),
        levels,
        add_tokens.clone(),
        ignore_tokens.clone(),
        add_before_ignore,
    );
    let mut titles: Vec<String> = Vec::new();
    if files.is_empty() {
        let title_paths = storage::get_note_titles(src_dir.clone());
        for path in title_paths {
            let path_name = match path.file_name() {
                Some(name) => name,
                None => {
                    eprintln!("Error parsing file name!");
                    continue;
                }
            };
            titles.push(path_name.to_string_lossy().to_string());
        }
    } else {
        for file in files {
            titles.push(file.to_string());
        }
    }

    let old_path = src_dir.clone();
    let new_path = imports_dir.clone();
    let mut counter = 0;

    let mut counter_pretty_print_new_line = confirm;
    for title in titles {
        let old_location = old_path.join(&title);
        if !old_location.is_file() || title.starts_with('.') {
            continue;
        }

        let extension = match PathBuf::from(&title).extension() {
            Some(extension) => match extension.to_str() {
                Some(string) => match string {
                    "txt" => "txt",
                    "md" => "md",
                    _ => continue,
                },
                None => continue,
            },
            None => "",
        };

        let new_title = get_new_title(tokens.clone(), title.clone(), ignore_tokens.clone());
        let new_location = new_path.join(&new_title);

        let keywords: Vec<&str> = new_title.split('.').collect();
        let keywords: Vec<&str> = keywords[0].split('_').collect();
        let file_already_exist = new_location.is_file();
        let input = cli::import_confirmation(
            title,
            keywords.clone(),
            confirm,
            overwrite,
            file_already_exist,
        );

        if file_already_exist && !overwrite {
            counter_pretty_print_new_line = true;
        }

        match input.trim() {
            "q" => {
                break;
            }
            "e" => {
                let path_to_tmp_file = env::temp_dir().join("keyword_edit");

                if !storage::enumerate_kw_editor_file(path_to_tmp_file.clone(), keywords) {
                    continue;
                }

                editor::open_editor(path_to_tmp_file.clone());

                let keywords = storage::read_kw_editor_file(path_to_tmp_file);
                let binding = keywords.join("_");
                let mut edited_location = new_path.join(binding);
                edited_location.add_extension(extension);

                if edited_location.is_file() {
                    eprintln!("  File with matching keywords already exists!\nSkipping file...");
                    continue;
                }

                storage::copy_file(old_location, edited_location);
                counter += 1;
            }
            "y" => {
                storage::copy_file(old_location, new_location);
                counter += 1;
            }
            _ => {
                println!("Skipping file...");
                continue;
            }
        }
    }

    println!(
        "{}Imported {} file{}",
        if counter_pretty_print_new_line {
            "\n"
        } else {
            ""
        },
        counter,
        if counter == 1 { "" } else { "s" }
    );
}

fn tokenize(
    src_dir: PathBuf,
    levels: u8,
    add_tokens: HashSet<String>,
    ignore_tokens: HashSet<String>,
    add_before_ignore: bool,
) -> HashSet<String> {
    let mut tokens: HashSet<String> = HashSet::new();
    let mut file_tokens: Vec<&std::path::Path> = src_dir.ancestors().collect();
    file_tokens.pop();

    if add_before_ignore {
        for token in &add_tokens {
            if !ignore_tokens.contains(token.as_str()) {
                tokens.insert(token.to_lowercase().to_string());
            }
        }
    }

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

        if !ignore_tokens.contains(token) {
            let token_split: Vec<&str> = token.split('_').collect();
            for token in token_split {
                if !ignore_tokens.contains(token) {
                    tokens.insert(token.to_lowercase().to_string());
                }
            }
        }
    }

    if !add_before_ignore {
        tokens.extend(add_tokens.clone());
    }

    tokens
}

fn get_new_title(
    mut curr_tokens: HashSet<String>,
    title: String,
    ignore_tokens: HashSet<String>,
) -> String {
    let mut new_title = String::new();

    let title_tokens: Vec<&str> = title.split('.').collect();
    let title_tokens: Vec<&str> = title_tokens[0].split('_').collect();
    for token in &title_tokens {
        curr_tokens.insert(token.to_lowercase().to_string());
    }

    let exhaustable_tokens = curr_tokens.clone();
    let mut new_title_tokens: Vec<String> = exhaustable_tokens
        .difference(&ignore_tokens)
        .cloned()
        .collect();
    new_title_tokens.sort();
    let new_title_tokens = new_title_tokens.join("_");
    new_title.push_str(new_title_tokens.as_str());

    let file = PathBuf::from(&title);
    if let Some(extension) = file.extension() {
        match extension.to_str() {
            Some(string) => {
                new_title.push('.');
                new_title.push_str(string);
            }
            None => {
                fatal("Invalid extension!");
            }
        }
    };

    new_title
}
