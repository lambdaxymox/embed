use embed::embed;


/// Embed a file into the binary as a vector using the `embed` macro and 
/// then print the result.
fn main() {
    let obj = embed!("../assets/triangle.obj");
    println!("{:?}", obj);
}
