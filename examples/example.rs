use embed::embed;


fn main() {
    let obj = embed!("../assets/triangle.obj");
    println!("{:?}", obj);
}
