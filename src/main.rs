use hey::{cli, storage, task};
use std::env;

fn main() {
    storage::initialize_storage();

    let args: Vec<String> = env::args().collect();

    let task = cli::parse_args(&args);

    task::do_task(task);
}
