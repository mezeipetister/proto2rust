# Proto2Rust converter

## Usage

```console
./proto2rust `input proto file` `output dir`
```

## Example

1. Create an example directory (`/example`)
2. Create a demo proto file (`/example/demo.proto`)
3. Copy proto2rust next to the .proto file, inside the `example` folder
4. Run the following command from shell

```console
./proto2rust demo.proto compiled_code/
```

The code above will read the `demo.proto` file, compile it into rust library,

create the `compiled_code` folder, and paste the rust code there.

## Compile the app

Run the following command:

```console
cargo build --release
```

You can find the built app inside the `target/release/` folder

Huge thank you for `protoc-rust` crate: <https://github.com/stepancheg/rust-protobuf/tree/master/protoc-rust/>