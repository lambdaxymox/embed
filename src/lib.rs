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
