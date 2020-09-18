# Key Commands in Rust [Unix Env]

## Creating a new project

We can create a new project repository, including the correct directory structure with:

	cargo init

## Building a project

To build our project without running it, run:

	cargo build

When we want to build our project and optimise it, run it with the release profile:

	cargo build --release

Use the normal cargo build for quick checking of the code. Use the release argument to optimise the code to the maximum possible that the Rust compiler will allow.

We call --release a profile, specifically the release profile. The Rust compiler has different levels of optimisation depending on what you want.

## Running a project 

Running a project can be done from the command line with:

	cargo run

Alternatively, if you've already run "cargo build" you can target the binary directly using: 

	./target/debug/binary_name

This does the same thing as "cargo run" but using two commands.

