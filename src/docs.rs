pub fn version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

pub fn help() {
    println!("Usage:");
    println!("  hey [keywords...]");
    println!("  hey . [keywords...]");
    println!("  hey [options]\n");

    println!("Options:");

    println!("  GENERAL");
    println!("    -v, --version\t\tDisplay version information and exit");
    println!("    -h, --help\t\t\tShow this message");
    println!("    --help-verbose\t\tShow a more descriptive help message");

    println!("\n  NOTE CREATION");
    println!("    . , --add [keywords...]\tAdd a new note (e.g. 'hey . rust macros')");

    println!("\n  SEARCH");
    println!("    [keywords...]\t\tSearch for notes matching the given keywords");

    println!("\n  ENVIRONMENT VARIABLE");
    println!("    HEY_EDITOR\t\t\tEditor command used by hey (overrides VISUAL and EDITOR)");

    println!("\n  EXAMPLES");
    println!("    hey . rust ownership");
    println!("    hey rust traits");
    println!("    hey --help");
}

pub fn help_verbose() {
    println!("NAME");
    println!("    hey - local-first note retrieval tool for programmers\n");

    println!("SYNOPSIS");
    println!("    hey [keywords...]");
    println!("    hey . [keywords...]");
    println!("    hey [options]\n");

    println!("DESCRIPTION");
    println!("    hey is a local-first note retrieval tool designed for storing");
    println!("    and retrieving programmer notes.");
    println!();
    println!("    Notes are stored as plain files on the local machine.");
    println!("    Searches are currently performed against note titles.");
    println!();
    println!("    Note titles are treated as collections of retrieval");
    println!("    keywords. Users are encouraged to choose descriptive");
    println!("    titles that reflect the contents of the note.\n");

    println!("OPTIONS");
    println!("    -h, --help");
    println!("        Display a concise help message.");
    println!();
    println!("    --help-verbose");
    println!("        Display detailed documentation.");
    println!();
    println!("    -v, --version");
    println!("        Display version information and exit.");
    println!();
    println!("    ., --add [keywords...]");
    println!("        Create a new note using the supplied keywords");
    println!("        as its title.\n");

    println!("SEARCH");
    println!("    Any argument sequence not interpreted as an option");
    println!("    is treated as a search query.");
    println!();
    println!("    Search results are ranked according to the number");
    println!("    of matching title keywords.\n");

    println!("ENVIRONMENT");
    println!("    HEY_EDITOR");
    println!("        Editor command used by hey.");
    println!();
    println!("        This variable takes precedence over VISUAL");
    println!("        and EDITOR.");
    println!();
    println!("    VISUAL");
    println!("        Used when HEY_EDITOR is not set.");
    println!();
    println!("    EDITOR");
    println!("        Used when neither HEY_EDITOR nor VISUAL");
    println!("        are set.\n");

    println!("EDITOR SELECTION");
    println!("    hey chooses an editor using the following order:");
    println!();
    println!("        1. HEY_EDITOR");
    println!("        2. VISUAL");
    println!("        3. EDITOR");
    println!("        4. Platform default");
    println!();
    println!("    Platform defaults:");
    println!("        Unix-like systems : vi");
    println!("        Windows           : notepad\n");

    println!("FILES");
    println!("    hey stores notes inside an operating-system");
    println!("    appropriate application data directory.");
    println!();
    println!("    The exact location depends on the platform.");
    println!();
    println!("    Linux");
    println!("        ~/.local/share/hey");
    println!();
    println!("    Windows");
    println!("        LocalAppData\\hey");
    println!();
    println!("    macOS");
    println!("        ~/Library/Application Support/hey\n");

    println!("EXAMPLES");
    println!("    Create a note:");
    println!("        hey . rust ownership");
    println!();
    println!("    Search for notes:");
    println!("        hey rust ownership");
    println!();
    println!("    Display help:");
    println!("        hey --help");
    println!();
    println!("    Display detailed help:");
    println!("        hey --help-verbose\n");

    println!("EXIT STATUS");
    println!("    0");
    println!("        Successful execution.");
    println!();
    println!("    Non-zero");
    println!("        An error occurred.\n");

    println!("SEE ALSO");
    println!("    https://github.com/oneTenCubed/hey");
}
