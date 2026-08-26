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
    println!("    . , --new [keywords...]\tAdd a new note (e.g. 'hey . rust macros')");
    println!("    Additional flags:");
    println!("    \t\t-md\t\tCreated note will have markdown extension");

    println!("\n  SEARCH");
    println!("    [keywords...]\t\tSearch for notes matching the given keywords (Shows prompt)");
    println!("    Additional flags:");
    println!("    \t\t-s, --search\tDisables prompt");

    println!("\n  IMPORT");
    println!(
        "    -i, --import [files...]\tCopy external notes to hey's local directory to enable it's services"
    );

    println!("\n  ENVIRONMENT VARIABLE");
    println!("    HEY_EDITOR\t\t\tEditor command used by hey (overrides VISUAL and EDITOR)");
    println!("    HEY_DEFAULT_EXT\t\tDefault extension to be set for created notes");

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
    println!("    ., --new [keywords...]");
    println!("        Create a new note using the supplied keywords");
    println!("        as its title.");
    println!("        Additional flags:");
    println!("            -md");
    println!("                Created notes will be set to have the");
    println!("                markdown extension.\n");

    println!("SEARCH");
    println!("    Any argument sequence not interpreted as an option");
    println!("    is treated as a search query. By default, enters");
    println!("    into an interactive prompt.");
    println!("        Additional flags:");
    println!("            -s, --search");
    println!("                Disables the interactive prompt.");
    println!();
    println!("    Search results are ranked according to the number");
    println!("    of matching title keywords.\n");

    println!("IMPORT");
    println!("    -i, --import [files...]");
    println!("        Copy external notes to hey's local directory");
    println!("        to enable it's services.");
    println!();
    println!("    -ic, --import-confirm [files...]");
    println!("        Copy external notes to hey's local directory");
    println!("        to enable it's services. Asks for confirmation");
    println!("        each time it copies a suitable file.");
    println!();
    println!("    Specify file name(s) after the import flag");
    println!("    to import them else, scans the current directory");
    println!("    and imports all suitable files.");
    println!();
    println!("    -l, --levels");
    println!("        Use this flag and give a number as an argument");
    println!("        to add the specified number of ancestor directory");
    println!("        names as keywords.");
    println!();
    println!("    --overwrite");
    println!("        If a file generates keywords that mathes an");
    println!("        existing file, use this flag to overwrite");
    println!("        without confirmation.");
    println!();
    println!("    --ignore");
    println!("        Specify the words to be ignored during keyword");
    println!("        generation.");
    println!();
    println!("    --add");
    println!("        Specify the words to be added during keyword");
    println!("        generation.\n");

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
    println!("        are set.");
    println!();
    println!("    HEY_DEFAULT_EXT");
    println!("        Used to set the default extension for");
    println!("        created notes.");
    println!("            Example usage:");
    println!("                HEY_DEFAULT_EXT=\".md\"\n");

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
