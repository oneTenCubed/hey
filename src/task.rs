use crate::{cli, search, storage};

#[derive(Clone, Debug)]
pub enum Command {
    Search,
    New,
    Editor,
    Cat,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub cmnd: Command,
    pub input: Option<String>,
}

fn validate_task(task: Result<Task, ()>) -> Task {
    match task {
        Ok(t) => t,
        Err(_) => {
            panic!("Invalid command!");
        }
    }
}

pub fn do_task(task: Result<Task, ()>) {
    let task = validate_task(task);
    match task.cmnd {
        Command::New => {
            storage::new_article(cli::get_title());
        }
        Command::Search => match task.input {
            Some(s) => {
                search::search(s);
            }
            None => (),
        },
        Command::Editor => match task.input {
            Some(s) => {
                storage::open_editor(s);
            }
            None => (),
        },
        Command::Cat => match task.input {
            Some(s) => {
                storage::cat_article(s);
            }
            None => (),
        },
    }
}
