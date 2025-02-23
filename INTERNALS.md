# Internal API

Here will be a quick overview of the crate, functions and global variables roles and interdependencies explanation.

## Architectural choices

### Synchronous rust

Rust async is like learning rust another time.
Althought I love using async,
I want to keep the code as simple as possibe, and found that async wasn't needed here.

Keeping the code synchronous allow for better error catching
and fresh contributors understanding.

## Browsing the source code (must read)

You can find READMEs everywhere for better crate by crate exploration.

# The source code structure

The pipelight source code is splitted into 5 crates.
Every crate serves the one where all the logic happens,
the **pipeline** crate.

## Cast crate

Mostly a wrapper around serde to get a config file parsed as Rust structs.
And further check what is inside for type safty.

## Utils crate

Contains utility functions to lessen the pain when doing some trivial
things across the source code.

They are simple to use because they higly abstract the logic beneath.

Divided in 3 main directories.

- Git
- Logger
- Teleport

### Git

Contains functions to:

- detect the git directory
- create and ensure pipelight git hooks

### Logger

Contains functions to:

- create and ensure a logger

### Teleport

Certainly the MVP of the utils crate.

Contains functions to:

- Recursively search a file in through the fs
- Telepor back and forth to the file

### Dates

Abstraction over date convertion and duration computation.

### Files

Abstraction over file reading, filepath (std::path::Path) usage...

# Functionning

When running a pipeline.
This happens.

Read config file -> Create a Inner Object (Pipeline struct) -> Run processes in the defined order while logging.

### Configuration file reading

The purpose of the first set of functions called when executing a CLI command,
is to find, read and parse the pipelight configuration file.

### Find

The Teleport crate is a kind of [cosmiconfig](https://github.com/cosmiconfig/cosmiconfig).
You call `Teleport::search("filename")` and it recursively seeks a file based on globbing pattern you provided.

The advantages it provide is that it can change the process cwd to where the configuration file lays and change it again to the previous cwd.

This is a back and forth teleportation to the folder that contain our file of interest.

```rs
let mut portal = Teleport::new().preffix("pipelight");
portal.search()?;

// Teleport process to file path
portal.teleport();
// Teleport process to where it was originaly launched
portal.origin();

```

**The Teleport crate may be the first internal crate to be publicly released to crate.io.**

### Read and Parse
