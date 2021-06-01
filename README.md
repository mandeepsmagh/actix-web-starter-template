# actix-web starter template

actix-web example that is more than just hello_world and also that is not too complex to understand for Rust / actix-web beginners.

Replace `template` with your project-name and you have initial project structure.

## Pre-Requisite:

- [Rust](https://www.rust-lang.org/tools/install)
- Install relevant plugins for VsCode - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) / IntelliJ - [IntelliJ Rust](https://www.jetbrains.com/rust/)
- Alternatively use provided devcontainer as reproducible dev setup which makes it easy to get started

## How to build:

`cargo build`

## How to run:

`cargo run`

Use to build and run binary application in single step.

## How to test:

`cargo test`

Above command runs all unit tests (embedded test modules) and API integration tests located under `src/tests` path.

## API test, once app is running:

`curl -v http://127.0.0.1:8000/health_check`
