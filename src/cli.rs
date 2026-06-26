use crate::{app::fatal, docs, editor, import, search, storage};
use std::{
    collections::HashSet,
    env, fs,
    io::{self, Write},
};

pub fn dispatcher() {
    let args: Vec<String> = env::args().collect();

    let _invoke = &args[0];
    // Maybe needed in the future, like warnings for using deprecated invoke commands

    if args.len() < 2 {
        docs::help();
        return;
    }
    let args: Vec<String> = args[1..].to_vec();

    if args[0] == "." || args[0] == "--add" {
        let title = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            get_title()
        };

        storage::new_article(title);
    } else if &'-'
        == match &args[0].chars().nth(0) {
            Some(val) => val,
            _ => &'+',
        }
    {
        match args[0].as_str() {
            "-h" | "--help" => {
                docs::help();
            }
            "-v" | "--version" => {
                docs::version();
            }
            "--help-verbose" => {
                docs::help_verbose();
            }
            "-i" | "--import" | "-ic" | "--import-confirm" => {
                let mut level_flag_index = None;
                let mut ignore_flag_index = None;

                for (index, arg) in args.iter().enumerate() {
                    if arg == "--ignore" {
                        ignore_flag_index = Some(index);
                    } else if arg == "-l" || arg == "--levels" {
                        level_flag_index = Some(index);
                    }
                }

                import::import(
                    match level_flag_index {
                        Some(index) => {
                            let level = &args[index + 1];
                            let level: u8 = level.parse().expect("  Invalid level!");

                            level
                        }
                        None => 0,
                    },
                    match ignore_flag_index {
                        Some(index) => {
                            let mut set = HashSet::new();
                            if ignore_flag_index > level_flag_index {
                                for arg in &args[(index + 1)..] {
                                    set.insert(&arg[..]);
                                }
                            }
                            set
                        }
                        None => HashSet::new(),
                    },
                    match args[0].as_str() {
                        "-i" | "--import" => false,
                        "-ic" | "--import-confirm" => true,
                        _ => {
                            unreachable!();
                        }
                    },
                );
            }
            /*"-l" => {
                todo!("Link functionality coming soon!");
            }
            "-z" => {
                todo!("Fuzzy searching coming soon!");
            }
            "-s" => {
                todo!("Synonym/abbrevation searching coming soon!");
            }*/
            _ => {
                println!("Invalid flag!\n");
                docs::help();
            }
        }
    } else {
        search::search(args.join(" "));
    }
}

// TODO: Improve title acceptance logic, make a format for title: validate_title() -> String
fn get_title() -> String {
    print!("Enter a title: ");
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

pub fn search_result(result_arr: Vec<search::Field>) {
    let mut input = String::new();
    let title: String;

    if result_arr.is_empty() {
        println!("  No matches!");
        return;
    } else if result_arr.len() == 1 {
        println!("  Exactly one match found: {}", result_arr[0].display_name);
        title = result_arr[0].file.clone();
    } else {
        for (index, field) in result_arr.iter().enumerate() {
            println!("  {}. {}", index + 1, field.display_name);
        }

        println!("");
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
            if input == "" || input == "q" || input == "Q" {
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
                    title = result_arr[n - 1].file.clone();
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
        "r" | "R" | "" => read_article(title),
        "w" | "W" => editor::open_editor(title),
        _ => println!("  Invalid input!"),
    }
}

fn read_article(title: String) {
    let path_to_file = storage::get_hey_notes_dir().join(title);

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
