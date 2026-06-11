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
* Plain text files
* Human-readable storage
* Unix-inspired workflow
* No AI or machine learning
* Deterministic behavior
* Simple and composable

The original note files remain accessible without hey.

## Current Features

### Note Creation

Create a new note by providing a title.

The title acts as a collection of search keywords that can later be used to retrieve the note.

### Editor Integration

hey respects standard Unix editor variables:

1. `VISUAL`
2. `EDITOR`

If neither is set, a fallback editor is used.

### Keyword Search

Search for notes using one or more keywords.

Results are ranked according to title matches.

Example:

```text
hey rust traits
hey asm directives
```

## Storage

Notes are stored locally under:

```text
~/.local/share/hey
```

Current versions store notes directly within hey-managed storage. [^*]

[^*]: hey currently targets Linux systems and follows common Unix conventions.

## Roadmap

### Planned

* Note preview during search
* Improved search result interaction
* Linking external notes through symbolic links
* Importing existing notes into hey

### Possible Future Improvements

* Better note organization
* Additional retrieval workflows
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

Current version: 0.1.0

hey is a personal project built as part of learning Rust, systems programming, and software architecture.

## Feedback

Discussions, ideas, and design feedback are welcome.

Pull requests are currently not accepted.

## License

MIT
