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

#[allow(dead_code)]
const EMBED_PATH: &str = "/embed/";


/// The `embed_bytes` macro includes a file as a vector of bytes. It loads the file relative 
/// to the root path of the assets `embed` generated in the build phase. this macro will yield a
/// `Vec<u8>` expression which is the contents of the file as raw bytes.
#[macro_export]
macro_rules! embed_bytes {
    ($asset_path:expr) => {{
        let bytes = include_bytes!(concat!(env!("OUT_DIR"), EMBED_PATH, $asset_path));
        let length = bytes.len();
        let mut vec: Vec<u8> = vec![0; length];
        for i in 0..length {
            vec[i] = bytes[i];
        }

        vec
    }}
}

/// This macro includes a utf8-encoded file as a string. The file is loaded 
/// relative to the root path of the assets `embed` generated in the build phase. 
/// This macro will yield an expression of type `&'static str` which is the
/// contents of the file.
#[macro_export]
macro_rules! embed_str {
    ($asset_path:expr) => {
        include_str!(concat!(env!("OUT_DIR"), EMBED_PATH, $asset_path))
    }
}

/// The `embed` macro includes a file as a vector of bytes. It loads the file relative 
/// to the root path of the assets `embed` generated in the build phase. this macro will yield a
/// `Vec<u8>` expression which is the contents of the file as raw bytes.
#[macro_export]
macro_rules! embed {
    ($asset_path:expr) => {
        include!(concat!(env!("OUT_DIR"), EMBED_PATH, $asset_path));
    }
}
