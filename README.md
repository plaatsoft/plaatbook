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

-   Watch, build and run server:

    ```sh
    make -C server start
    ```

-   Run CI checks:

    ```sh
    make -C server ci
    ```

### Web

-   Install [Node.js](https://nodejs.org/en/download)
-   Watch, build and run web frontend:

    ```sh
    make -C web start
    ```

-   Run CI checks:

    ```sh
    make -C web ci
    ```

-   Open page in browser

## License

Copyright © 2024 [PlaatSoft](https://www.plaatsoft.nl/)

Licensed under the [MIT](LICENSE) license.
