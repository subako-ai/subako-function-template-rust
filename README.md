# Subako function package template (Rust)

A starting point for a [Subako](https://subako.ai) function package written in Rust.

```sh
cargo generate subako-ai/subako-function-template-rust --name hello
cd hello
```

## Write functions

Each `#[function]` is a tool the agent can call:

- the function's name is the tool's name;
- its doc comment is the description the model reads;
- its input and output types, which derive `JsonSchema`, become the tool's schemas.

`package!` lists the functions and generates `main`, which serves them as an MCP server over stdio (Subako's function ABI 1). Inside a call, the session's Volume is at `/volume/tmp` and `/volume/data`, and the package's own `assets/` directory is at `/package`. Make HTTP requests with `subako_function_sdk::http`, and show the model an image from the Volume by returning an `Answer` with `with_image_file`.

## Test

`cargo test` runs your functions as plain Rust.

## Package and upload

```sh
./package.sh                                   # dist/hello.tar.gz
subako function create dist/hello.tar.gz       # first version
subako function push <function_id> dist/hello.tar.gz   # later versions
```

Then grant the package to an agent under **Function packages** in the agent version editor.

`package.sh` needs the `wasm32-wasip1` target, which `rust-toolchain.toml` installs through rustup.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
