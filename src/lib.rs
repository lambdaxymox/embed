#[macro_export]
macro_rules! embed {
    ($asset:expr) => {
        let bytes_arr = include_bytes!(asset);
        let mut bytes_vec: Vec<u8> = vec![0; &bytes_arr[..].len()];
        for i in 0..bytes_vec.len() {
            bytes_vec[i] = bytes_arr[i];
        }

        bytes_vec
    }
}
