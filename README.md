# Rust-Elm-Template

> **NOTE**: This template is "production ready" (in the sense it'll work), but this README is very long, and the code has not been fully optimised. When this note box is gone, you can be assured that everything's neat and tidy!

A template for creating web-based desktop applications (similar to Electron, but not Electron).

It uses native WebView, in particular WebKit for Linux and macOS, and MSHTML on Windows.

Heavily inspired by [this kanban app](https://github.com/huytd/kanban-app).

-----

## About this project
It's basically a web view powered by Rust, with a front end powered by Elm. You don't _have_ to use Elm, feel free to use HTML, CSS and JavaScript as much as you like (If you check out the `main.rs` file, it should be pretty straight forward how to add/remove new bits and bobs). 

Yes, I'm aware that this template has a terrible name that doesn't describe what this project is whatsoever. Sorry.

This template includes a sample project that uses Bootstrap (declared in `main.rs`), some basic CSS (declared in `style.css`) and some Elm which is declared in `www/src/Main.elm`.

-----

## Things you'll need to use this template
Non-NixOS users:
- Elm, which can be downloaded [here](https://guide.elm-lang.org/install.html)
- Cargo, which can be downloaded [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

NixOS users:
- Nothing: the `./compile` file is powered by the `nix-shell` and should download the packages as needed to compile it.

## Setting up your project
There are a few things to do to set this project up as your own, but trust me, it's really straight forward, all you have to do is change the contents of the `Cargo.toml` file:

- `name` - The name of your project (and output executable)
- `description` - A description of your project
- `authors` - You probably don't want my name plastered over your project

## Building the application
If you're on NixOS, you're in luck! Just run `./compile` in this root directory and it'll compile everything (The Elm code and the binary written in Rust). If you're not on NixOS, you're going to have to do a bit of work:

- Navigate to the `/www` folder
- Run `./compile-elm` to compile the elm code
- Return back to the root folder
- Run `cargo build --release`

## Running the application
Head down to `target/release/<your_project_name>` and run it!

### NixOS TLS/SSL errors
This may only apply to a small 1% of people, but I think it's noteworthy because this took me hours to resolve. If you're running NixOS and are trying to run an application that accesses another webpage (For example, the example in this repository which uses a stylesheet hosted by Bootstrap), and you're not running a Gnome desktop, there is a small chance that you'll get the following error message:

```
TLS/SSL support not available; install glib-networking
```

To resolve this, add the following file from [here](https://github.com/JorelAli/nixos/blob/master/glib-networking.nix) into your `/etc/nixos` directory (where your `configuration.nix` file is), then add it to your `imports`, as shown:

```
imports = [
  ./hardware-configuration.nix        # Import hardware configuration
  ./glib-networking.nix
];
```
