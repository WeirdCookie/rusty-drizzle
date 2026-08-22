# rusty-drizzle

A basic 2D rain generator using rust

#### AI Disclosure

This project was mostly human written, but AI was also involved for the following things:

- debugging compile errors (e.g., `image`/`getrandom` wasm issues)

- initial project scaffolding (Cargo, wasm-pack setup)

- adding the gif feature into the CLI part

- implementing the adjustable rain parameters (size, speed, count, angle, width) in the web UI and CLI

All final commits and decisions were made by a human.

The core logic and design decisions were reviewed and verified by the author

## requirements

you only need those for building from source

- newest version of rust from https://rust-lang.org/tools/install/
- git from https://git-scm.com/install/
- curl

## How to use

#### If you want to to just test it on your webbrowser simply go on 
https://rusty-drizzle.weirdcookie.workers.dev/



#### On Windows

Go to the [releases](https://github.com/WeirdCookie/rusty-drizzle/releases) page and download the latest `.exe`, then double-click it (or run it from a terminal).

#### On GNU/Linux

There is no pre-built Linux binary yet, so you build it from source. You only need Rust installed (see [requirements](#requirements)).

```bash
git clone https://github.com/WeirdCookie/rusty-drizzle.git && cd rusty-drizzle && cargo build --release
```

Then run it:

```bash
./target/release/rusty-drizzle
```

Run with `--help` to see all options:

```bash
./target/release/rusty-drizzle --help
```

Rendering a 10 second GIF with custom parameters:

```bash
./target/release/rusty-drizzle --size 20 --speed 500 --count 400 --angle 30 --width 3
```

## Examples

This could be an image of v0.1.0


![Example image v0.0.1](random_pattern.png)




This could be a gif of v1.0.0


![Example GIF v1.0.0](rain.gif)

