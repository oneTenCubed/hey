pub fn version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

pub fn help() {
    println!("Usage:");
    println!("  hey [options] [keywords]\n");

    println!("Options:");
}
