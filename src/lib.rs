//! # Embed
//! This repository exports a macro for embedding static art assets into Rust binaries.
//! 
//! ## Usage
//! To use the embed macro in your application, add the following line to your Rust dependencies.
//! ```text
//! // ...
//! embed = { git = "https://github.com/lambdaxymox/embed" }
//! // ...
//! ```
//! 
//! ## Examples
//! Here is an example of using the `embed` macro in a working program.
//! ```rust
//! use embed::embed;
//! 
//! fn main() {
//!     let obj = embed!("../assets/triangle.obj");
//!     println!("{:?}", obj);
//! }
//! ```
//! 

/// The embed macro takes a file path, statically loads the file, and generates the code 
/// to drop it into a binary vector at runtime. 
#[macro_export]
macro_rules! embed {
    ($asset:expr) => {{
        let bytes = include_bytes!($asset);
        let length = bytes.len();
        let mut vec: Vec<u8> = vec![0; length];
        for i in 0..length {
            vec[i] = bytes[i];
        }

        vec
    }}
}
