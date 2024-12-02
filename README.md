# PlaatBook

PlaatBook is a X/Twitter like clone example project

> [!IMPORTANT]
> PlaatBook is currently in development and not finished yet

## Getting Started

### Server

-   Install the latest Rust toolchain with [rustup](https://rustup.rs/)
-   Install `cargo-udeps`, `cargo-deny` and `cargo-watch`:

    ```sh
    cargo install cargo-udeps cargo-deny cargo-watch
    ```

-   Build and run server:

    ```sh
    cd server; cargo watch -x run
    ```

-   Run CI checks:

    ```sh
    make -C server ci
    ```

## License

Copyright © 2024 [PlaatSoft](https://www.plaatsoft.nl/)

Licensed under the [MIT](LICENSE) license.
