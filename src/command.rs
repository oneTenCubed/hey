use crate::{app::fatal, cli, docs, import, search, storage};
use std::{collections::HashSet, env};

// TODO: add -n flag to limit search results (by default 10) and add --all to display all
// add a default search result limit env variable
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
            cli::get_title()
        };

        storage::new_article(title); // TODO: support ".md" through -md
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
                parse_import(args);
            }
            /*"-s" => {
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

struct ImportArgs<'a> {
    confirm: bool,
    overwrite: bool,
    ignore: HashSet<&'a str>,
    levels: u8,
    files: HashSet<&'a str>,
}

fn parse_import(args: Vec<String>) {
    let mut state = ImportArgs {
        confirm: false,
        overwrite: false,
        ignore: HashSet::new(),
        levels: 0,
        files: HashSet::new(),
    };

    let mut ignore_flag_index = None;

    for (index, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-ic" | "--import-confirm" => state.confirm = true,
            "-l" | "--levels" => {
                state.levels = match args[index + 1].parse() {
                    Ok(num) => num,
                    Err(_) => fatal("Invalid argument for level!"),
                }
            }
            "--overwrite" => state.overwrite = true,
            "--ignore" => ignore_flag_index = Some(index + 1),
            _ => (),
        }
    }

    match ignore_flag_index {
        Some(index) => {
            if args.len() <= index || args[index].chars().nth(0) == Some('-') {
                eprintln!("Ignore field is empty. Nothing ignored...");
                ()
            }

            for arg in &args[index..] {
                if arg.chars().nth(0) == Some('-') {
                    break;
                }

                state.ignore.insert(arg);
            }
        }
        None => (),
    }

    if args[1].chars().nth(0) != Some('-') {
        for arg in &args[1..] {
            if arg.chars().nth(0) == Some('-') {
                break;
            }

            state.files.insert(arg);
        }
    }

    import::import(
        state.levels,
        state.ignore,
        state.confirm,
        state.overwrite,
        state.files,
    );
}
