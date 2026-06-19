use crate::{app::fatal, docs, editor, search, storage};
use std::{
    env,
    io::{self, Write},
};

pub fn dispatcher() {
    let args: Vec<String> = env::args().collect();

    let _invoke = &args[0];
    // Maybe needed in the future, like warnings for using deprecated invoke commands

    if args.len() < 2 {
        docs::help();
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
            /*"-m" => {
                todo!("Migrate functionality coming soon!");
            }
            "-l" => {
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
    let mut title = String::new();

    if result_arr.is_empty() {
        println!("No matches!");
        return;
    } else if result_arr.len() == 1 {
        println!("Exactly one match found: {}", result_arr[0].file);
        title = result_arr[0].file.clone();
    } else {
        for (index, field) in result_arr.iter().enumerate() {
            println!("  {}. {}", index + 1, field.file);
        }

        print!("\n:: Enter file number to interact: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Error reading!");

        if input.trim() == "" {
            return;
        }

        let n = input.trim().parse::<usize>().expect("Invalid input!");
        match n <= result_arr.len() {
            true => {
                title = result_arr[n - 1].file.clone();
            }
            false => {
                println!("Invalid input!");
            }
        }
    }

    input.clear();
    print!(":: Display content (r) OR open in editor (w)? [R/w] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Error reading!");

    match input.trim() {
        "r" | "R" | "" => storage::read_article(title),
        "w" | "W" => editor::open_editor(title),
        _ => println!("Invalid input!"),
    }
}
