use crate::task;
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
    let title = title.join("_");

    title
}

pub fn get_title() -> String {
    print!("Enter a title: ");
    io::stdout().flush().unwrap();

    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Error reading!");

    parse_title(title)
}
