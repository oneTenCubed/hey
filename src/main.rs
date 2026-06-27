use oneTenCubed_hey::{command, storage};

fn main() {
    storage::initialize_storage();

    command::dispatcher();
}
