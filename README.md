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
hey .
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

Current versions store notes directly within hey-managed storage. [^3]
[^3]: hey currently targets Linux systems and follows common Unix conventions. As such, the program should work on most Linux based systems and Unix like systems. The latter is untested.

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

## Project Status

Current version: 0.2.3

hey is a personal project built as part of learning Rust, systems programming, and software architecture.

## Feedback

Discussions, ideas, and design feedback are welcome.

Pull requests are currently not accepted.

## License

MIT
