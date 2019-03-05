# Embed
This repository exports a macro for embedding static art assets into Rust binaries.

## Usage
To use the embed macro in your application, add the following line to your Rust dependencies.
```
embed = { git = "https://github.com/stallmanifold/embed" }
```

## Examples
Here is an example of using the `embed` macro in a working program.
```rust
use embed::embed;

fn main() {
    let obj = embed!("../assets/triangle.obj");
    println!("{:?}", obj);
}
```
Compile and run this example by running
```bash
cargo run --example example
```

## Dependencies
This repository has no external dependencies. It requires any version of Rust 2018 edition to use.
