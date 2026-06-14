use oneTenCubed_hey::{cli, storage};

fn main() {
    storage::initialize_storage();

    cli::dispatcher();
}
