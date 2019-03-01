# pastepin

Simple pastebin

## Compilation

You'll need [Rust](https://www.rust-lang.org/) and cargo installed (just use rustup).
Navigate to project's directory and compile with `cargo build --release`. After successful 
compilation, the final binary will be placedat `target/release/pastepin` (relative to the project 
directory).

## Configuration

The only thing you really will be needing to set is a database url. You can do that either through 
environment variable or a `Rocket.toml` file. You can read more about configuration options on the 
[rocket framework website](https://rocket.rs/v0.4/guide/configuration/#configuration).

### Rocket.toml

Minimal `Rocket.toml`. Needs to be placed in the same directory as the binary or it's parent 
directory (checking recursively up to file system root).

```
[global]
port = 80
address = "1.2.3.4"

[global.databases]
pastepin_db = { url = "postgresql://pastepin_user:password@1.2.3.4:5432/pastepin_db" }
```

### Environment variables

```
export ROCKET_DATABASES={pastepin_db={url="postgresql://pastepin_user:password@1.2.3.4:5432/pastepin_db"}}
export ROCKET_PORT=80
export ROCKET_ADDRESS="1.2.3.4"
./pastepin
```

