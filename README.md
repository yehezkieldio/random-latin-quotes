## Random Latin Quotes
> A service to serve random Latin quotes.

### Overview

This is a simple RESTful service that serves random Latin quotes. The quotes are stored in a json file. The service is built with the `actix-web` framework and uses the `serde` library for serialization and deserialization of the quotes, and the `rand` library for random number generation to select a quote.

**NOTE:** You may need nightly Rust to build this project, with the linker `mold` installed and the Cranelift codegen backend enabled. You can also disable or remove them in the `Cargo.toml` and `.cargo/config.toml` files.

### Usage

To run the service, simply clone the repository and run the following command:

```bash
cargo run
```

The service will be available at `http://127.0.0.1:8080`.

### License

Licensed under the MIT license. See the [LICENSE](LICENSE) file for details.