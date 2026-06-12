use crate::{search, task};
use std::io::{self, Write};

pub fn parse_args(args: &Vec<String>) -> Result<task::Task, ()> {
    let task: task::Task;

    if args.len() < 2 {
        return Err(());
    }

    if args[1] == "." {
        task = task::Task {
            cmnd: task::Command::New,
            input: None,
        }
    } else {
        task = task::Task {
            cmnd: task::Command::Search,
            input: Some(args[1..].join(" ")),
        }
    }

    Ok(task)
}

fn parse_title(title: String) -> String {
    let title: String = title.trim().parse().unwrap();
    let title: String = title.to_lowercase();
    let title: Vec<&str> = title.split(' ').collect();
    let title = title.join(" ");

    title
}

pub fn get_title() -> String {
    print!("Enter a title: ");
    io::stdout().flush().unwrap();

    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Error reading!");

    parse_title(title)
}

pub fn search_result(result_arr: Vec<search::Field>) {
    if result_arr.is_empty() {
        println!("No matches!");
        return;
    }

    for i in 0..result_arr.len() {
        println!("  {}. {}", i + 1, result_arr[i].file);
    }

    let mut input = String::new();
    print!("\n:: Enter file number to interact: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Error reading!");

    if input.trim() == "" {
        return;
    }

    let mut title = String::new();
    let n = input.trim().parse::<usize>().expect("Invalid input!");
    match n <= result_arr.len() {
        true => {
            title = result_arr[n - 1].file.clone();
        }
        false => {
            println!("Invalid input!");
        }
    }

    input.clear();
    print!(":: Display content (r) OR open in editor (w)? [R/w] ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Error reading!");

    let mode: char = match input.trim() {
        "" => 'r',
        "r" => 'r',
        "R" => 'r',
        "w" => 'w',
        "W" => 'w',
        _ => {
            println!("Invalid input!");
            return;
        }
    };

    task::do_task(Ok(task::Task {
        cmnd: match mode {
            'r' => task::Command::Cat,
            'w' => task::Command::Editor,
            _ => {
                println!("");
                return;
            }
        },
        input: Some(title),
    }));
}
