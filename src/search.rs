use crate::{app::fatal, cli, storage};

#[derive(Clone)]
pub struct Field {
    pub matches: i32,
    pub file: String,
    pub display_name: String,
}

pub fn search(s: String) {
    let binding = s.to_lowercase();
    let search_tokens: Vec<&str> = binding.split(' ').collect();

    let titles: Vec<String> = storage::get_note_titles(storage::get_hey_notes_dir());
    let mut result: Vec<Field> = Vec::new();

    for title in titles {
        let file = title.clone();
        let binding: Vec<&str> = title.split(".").collect();
        let title_tokens: Vec<&str> = binding[0].split('_').collect();
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
            file: file,
            display_name: title_tokens.join(" "),
        });
    }

    let n = match result.len() {
        0 => {
            fatal("No notes have been created yet. Create new notes to start searching.");
        }
        val => val,
    };

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
