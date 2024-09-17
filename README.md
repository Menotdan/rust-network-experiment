# Rust Networking Experiment

This is a small rust project I made in the interest of potentially making a multiplayer game. This repo also contains a VSCode launch configuration file which lets you debug a server and client seperately.

You can also build the debug client and server yourself by using `cargo build --bin=network-project --package=network-project` for the client and adding `--features server` for the server (copied from the VSCode launch configuration).

Currently, new clients joining the server will not have other clients' data replicated. This project is also generally incomplete in other ways.
