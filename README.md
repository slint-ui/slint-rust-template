# SixtyFPS Rust Template

A template for a Rust application that's using [SixtyFPS](https://sixtyfps.io) for the user interface.

## About

This template helps you get started developing a Rust application with SixtyFPS as toolkit
for the user interface. It demonstrates the integration between the `.60` UI markup and
Rust code, how to trigger react to callbacks, get and set properties and use basic widgets.

## Usage

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.
2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
    ```
    cargo install cargo-generate
    ```
3. Set up a sample project with this template
    ```
    cargo generate --git sixtyfpsui/sixtyfps-rust-template --name my-project
    cd my-project
    ```
3. Build with cargo
    ```
    cargo build
    ```
4. Run the application binary
     ```
     cargo run
     ```

We recommend using an IDE for development, along with our [LSP-based IDE integration for `.60` files](https://github.com/sixtyfpsui/sixtyfps/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [SixtyFPS extension](https://marketplace.visualstudio.com/items?itemName=SixtyFPS.sixtyfps-vscode).

## Next Steps

We hope that this template helps you get started and you enjoy exploring making user interfaces with SixtyFPS. To learn more
about the SixtyFPS APIs and the `.60` markup language check out our [online documentation](https://sixtyfps.io/docs/rust/sixtyfps/).
