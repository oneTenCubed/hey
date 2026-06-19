hey(1) ["hey(1)" ["Documentation Manual"]]

# NAME

hey - local-first note retrieval tool for programmers

# SYNOPSIS

hey [keywords...]++
hey . [keywords...]++
hey [options]

# DESCRIPTION

*hey* is a local-first note retrieval tool designed for storing and
retrieving programmer notes.

Notes are stored as plain files on the local machine. Searches are
performed against note titles, which are treated as collections of
retrieval keywords.

Unlike full-text note search systems, *hey* currently searches note
titles only. Users are encouraged to choose descriptive titles that
reflect the contents of each note.

Example note titles:++
\	rust ownership++
\	rust traits++
\	asm directives++
\	linux namespaces

Search results are ranked according to the number of matching keywords.

The project prioritizes:
	- Local-first storage
	- Human-readable files
	- Deterministic behavior
	- Simplicity
	- Unix-inspired workflows

The project intentionally avoids:
	- Cloud services
	- AI-powered retrieval
	- Remote synchronization

# OPTIONS

## -h, --help

Display a concise help message and exit.

## --help-verbose

Display detailed documentation and exit.

## -v, --version

Display version information and exit.

## ., --add [keywords...]

Create a new note using the supplied keywords as its title.

Example:++
\	hey . rust ownership

# SEARCH

Any argument sequence not interpreted as an option is treated as a
search query.

Example:++
\	hey rust ownership

Searches are currently performed against note titles only.

# ENVIRONMENT

## HEY_EDITOR

Editor command used by *hey*.

If set, this variable takes precedence over all other editor
configuration.

## VISUAL

Used when `HEY_EDITOR` is not set.

## EDITOR

Used when neither `HEY_EDITOR` nor `VISUAL` are set.

# EDITOR SELECTION

Editors are selected using the following precedence order:++
\	1. HEY_EDITOR++
\	2. VISUAL++
\	3. EDITOR++
\	4. Platform default

Platform defaults:++
\	Unix-like systems : vi++
\	Windows           : notepad

# FILES

hey stores its data in an operating-system appropriate application data
directory.

## Typical locations:

\- *Linux*++
\	~/.local/share/hey++
\- *Windows*++
\	%LOCALAPPDATA%\hey++
\- *macOS*++
\	~/Library/Application Support/hey

Notes are stored within the application's managed notes directory.

# EXAMPLES

\- Create a note:++
\	hey . rust ownership++
\- Search for notes:++
\	hey rust ownership++
\- Display help:++
\	hey --help++
\- Display detailed help:++
\	hey --help-verbose

# EXIT STATUS

## 0

Successful execution.

## Non-zero

An error occurred.

# SEE ALSO

vi(1), vim(1), nvim(1)
