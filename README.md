# hey

A Unix-inspired personal knowledge retrieval tool for programmers.

## Motivation

I take notes while learning and working on projects. Over time, finding the right note became increasingly tedious:

* Navigating to the correct directory
* Finding the correct file
* Opening the file in an editor
* Reading the contents

hey aims to reduce this friction by making notes retrievable through keyword-based search.

## Philosophy

* Local-first
* Plain text files[^1]
* Human-readable storage
* Unix-inspired workflow
* No AI or machine learning
* Deterministic behavior
* Simple and composable

The original note files remain accessible without hey.
[^1]:Although not guaranteed, should work with any regular(-) file
## Current Features

### Note Creation

Run this command to add a note:

```bash
hey .   # or hey . title word
```

Create a new note by providing a title with matching keywords.

The title acts as a collection of search keywords that can later be used to retrieve the note.

### Editor Integration

hey respects standard Unix editor variables:

1. `VISUAL`
2. `EDITOR`

If neither is set, a fallback editor is used.

### Keyword Search

Search for notes using one or more keywords.[^2]
[^2]:Current system expects exact keyword matches. As such "word" and "words" don't match. Case-insensitive.

Results are ranked according to title matches.

Example:

for
```bash
hey rust traits
```

result:
```text
1, rust traits
2, rust ownership
```

i.e, exact match scores higher

## Storage

Notes are stored locally under:

Linux:
`$XDG_DATA_HOME/hey` OR `$HOME/.local/share/hey`

macOS:
`$HOME/Library/Application Support`

Windows:
`{FOLDERID_LocalAppData}`

Current versions store notes directly within hey-managed storage.

## Roadmap

### Planned

* Note preview during search
* Linking external notes through symbolic links
* Importing existing notes into hey

### Possible Future Improvements

* Better note organization
* Performance improvements where justified by real usage

## Non-Goals

At present, hey is not intended to be:

* A general-purpose note-taking platform
* A cloud service
* An AI assistant
* A collaborative knowledge base

## Installation

```bash
cargo install oneTenCubed-hey
```

See Troubleshooting in case of errors.

## Manpage

To generate a man page for hey,

1. Go to: **https://github.com/oneTenCubed/hey**
2. Find the **docs** directory
3. Open **hey.1** inside **man** directory
4. Download raw file
5. Find it in downloads, let: ~/downloads/hey.1
6. Install to system's manpage db by running the command
```bash
sudo install -Dm644 ~/downloads/hey.1 /usr/local/share/man/man1/hey.1
```

Now running `man hey` produces a proper manpage for hey

## Troubleshooting

### `cargo` command not working?

`cargo` is the package manager for rust programming language. To install `cargo`, it is recommended to install `rustup`. `rustup` is most probably available to install through your system's package manager, or else visit: https://rustup.rs/

### `hey` command not working after install through cargo?

The most probable reason is that `$PATH` isn't set to cargo's binary directory ($HOME/.cargo/bin). To add to path:

#### Linux & macOS
Assuming Cargo is in `~/.cargo/bin`

1. Add to your shell config:
- zsh: `~/.zshrc`
- bash: `~/.bash_profile` (or `~/.bashrc`)
- sh: `~/.shrc` 

2. Append:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
add it at the very end of the shell config)

3. Reload config:
```bash
source ~/.zshrc     # or: source ~/.bash_profile
```

4. Verify:
```bash
cargo --version
```

#### Windows (PowerShell)

1. Check it exists:
- `%USERPROFILE%\.cargo\bin`

2. Add it for your user:
```powershell
$old = [Environment]::GetEnvironmentVariable("Path","User") `
if ($old -notlike "*$env:USERPROFILE\.cargo\bin*") { `
  [Environment]::SetEnvironmentVariable("Path", "$old;$env:USERPROFILE\.cargo\bin", "User") `
}
```

3. Restart your terminal (or sign out/in).


If all steps are followed correctly, `hey` should run now. Check:
```bash
hey --version
```

### Another command with similar name `hey`?
    
The quickest fix is to give an alias to this `hey` by adding this line to your shell config.
```bash
alias name='$HOME/.cargo/bin/hey' 
# replace name with any alias you want like, heyy or note
```

## Platform Notes
- Linux
    
    1. Arch Linux x86_64
        `hey` was developed and initially tested in Arch Linux x86_64.

    2. Kali GNU/Linux Rolling x86_64
        `hey` has been tested successfully.
        Found quirks regarding binary with similar alias, see troubleshooting for help.

- Windows 11 (25H2) x86_64
    `hey` has been tested successfully in Windows 11 (25H2).
    Search related bug from v0.3.5 has been patched.

- MacOS (14 Sonoma) x86_64
    `hey` has been tested successfully in macOS 14 Sonoma x86_64.

- FreeBSD x86_64
    `hey` has been tested successfully in FreeBSD 14.3-RELEASE.
    Found rustc version lag when rust is installed through the package manager. Install rustup to get the latest rustc version, `hey` requires rustc v1.95. See "Troubleshooting" to get rustup website link.

- Android (Termux)
    `hey` has been tested successfully in Termux on Android 10 aarch64.
    Installation is currently undocumented. [^3]

[^3]: Contact for documentation.

## Project Status

Current version: 0.3.6

hey is a personal project built as part of learning Rust, systems programming, and software architecture.

## Feedback

Discussions, ideas, and design feedback are welcome.

Pull requests are currently not accepted.

## License

MIT
