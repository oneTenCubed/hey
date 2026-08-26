use crate::{app::fatal, editor, search};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

// TODO: Improve title acceptance logic, make a format for title: validate_title() -> String
pub fn get_title() -> String {
    print!("  Enter a title: ");
    io::stdout().flush().unwrap();

    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Error reading!");

    let title: String = title.trim().parse().unwrap();
    if title.is_empty() {
        fatal("Invalid title!");
    }
    let title: String = title.to_lowercase();

    title
}

pub fn non_interactive_search_result(result_arr: Vec<search::Field>) {
    if result_arr.is_empty() {
        println!("  No matches!");
    } else if result_arr.len() == 1 {
        println!("  Exactly one match found: {}", result_arr[0].display_name);
    } else {
        for (index, field) in result_arr.iter().enumerate() {
            println!("{:4}. {}", index + 1, field.display_name);
        }
    }
}

pub fn interactive_search_result(result_arr: Vec<search::Field>) {
    let mut input = String::new();
    let path_to_file: PathBuf;

    if result_arr.is_empty() {
        println!("  No matches!");
        return;
    } else if result_arr.len() == 1 {
        println!("  Exactly one match found: {}", result_arr[0].display_name);
        path_to_file = result_arr[0].file.clone();
    } else {
        for (index, field) in result_arr.iter().enumerate() {
            println!("{:4}. {}", index + 1, field.display_name);
        }

        println!();
        loop {
            print!(":: Enter file number to interact: ");
            io::stdout().flush().unwrap();
            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(_) => {
                    eprintln!("  Error reading!");
                    continue;
                }
            };

            input = input.trim().to_string();
            if matches!(input.as_str(), "" | "q" | "Q") {
                return;
            }

            let n = match input.parse::<usize>() {
                Ok(value) => value,
                Err(_) => {
                    eprintln!(
                        "  Invalid input! Expected 1-{} (or press Enter to exit)",
                        result_arr.len()
                    );
                    continue;
                }
            };

            match n <= result_arr.len() {
                true => {
                    path_to_file = result_arr[n - 1].file.clone();
                }
                false => {
                    eprintln!(
                        "  Invalid input! Expected 1-{} (or press Enter to exit)",
                        result_arr.len()
                    );
                    continue;
                }
            }

            break;
        }
    }

    input.clear();
    print!(":: Display content (r) OR open in editor (w) [Rw]? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("  Error reading!");

    match input.trim() {
        "r" | "R" | "" => read_article(path_to_file),
        "w" | "W" => editor::open_editor(path_to_file),
        "q" | "Q" => (),
        _ => println!("  Invalid input!"),
    }
}

fn read_article(path_to_file: PathBuf) {
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

pub fn import_confirmation(
    title: String,
    keywords: Vec<&str>,
    confirm: bool,
    overwrite: bool,
    file_already_exist: bool,
) -> String {
    let mut input = String::new();

    if keywords.is_empty() {
        return String::from("n");
    }

    if confirm {
        print!("\n  Importing file \"{title}\" with keywords:\n\t");
        for keyword in &keywords {
            print!("\"{}\" ", keyword);
        }
        if file_already_exist {
            print!("\n  File with matching keywords already exist!");
        }
        io::stdout().flush().unwrap();
        print!(
            "\n:: {} file (y) OR edit keywords (e) [yeN]? ",
            if file_already_exist {
                "Overwrite"
            } else {
                "Import"
            }
        );
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("  Error reading!");
    } else {
        if !overwrite && file_already_exist {
            print!("\n  File with matching keywords already exist!\n    Keywords: ");
            for keyword in &keywords {
                print!("\"{}\" ", keyword);
            }
            io::stdout().flush().unwrap();
            print!("\n:: Overwrite file (y) OR edit keywords (e) [yeN]? ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("  Error reading!");
        }
    }

    input
}
