## Random Latin Quotes
> A service to serve random Latin quotes

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)

### Overview

This is simple RESTful service that serves random Latin quotes. The quotes are stored in a json file and are served by a `actix-web` server, built with `Rust`.

**NOTE:** You may need nightly Rust to build this project, with the linker `mold` installed and the Cranelift codegen backend enabled. You can also disable or remove them in the `Cargo.toml` and `.cargo/config.toml` files.

### Usage

To run the service, simply clone the repository and run the following command:

```bash
cargo run
```

The service will be available at `http://127.0.0.1:8080`.

### License

Licensed under the MIT license. See the [LICENSE](LICENSE) file for details.