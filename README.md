# PlaatBook

<div>
<img align="left" src="web/public/images/icon-192x192.png" width="96" height="96" />
<br/>
<p>
    PlaatBook is a X/Twitter like clone example project. It is written in Rust and Preact. The backend mostly uses custom code from <a href="https://github.com/bplaat/crates">bplaat/crates</a>, and the frontend uses Bulma for styling.
</p>
<br/>
</div>

> [!IMPORTANT]
> PlaatBook is currently in development and not finished yet

## Getting Started

### Server

-   Install the latest Rust toolchain with [rustup](https://rustup.rs/)
-   Install nightly `rustfmt`, `cargo-deny`, `cargo-watch`, `cargo-nextest` and `openapi-generator`:

    ```sh
    rustup toolchain add nightly --component rustfmt
    cargo install cargo-deny cargo-watch cargo-nextest
    cargo install --git https://github.com/bplaat/crates.git openapi-generator
    ```

-   Watch, build and run server:

    ```sh
    make -C server start
    ```

-   Run checks:

    ```sh
    make -C server check
    ```

### Web

-   Install [Node.js](https://nodejs.org/en/download)
-   Watch, build and run web frontend:

    ```sh
    make -C web start
    ```

-   Run checks:

    ```sh
    make -C web check
    ```

-   Open page in browser

## License

Copyright © 2024-2025 [PlaatSoft](https://www.plaatsoft.nl/)

Licensed under the [MIT](LICENSE) license.
