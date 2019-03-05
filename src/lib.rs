#[inline]
fn __arr_to_vec(bytes: &[u8]) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = vec![0 as u8; length];
    for i in 0..length {
        vec[i] = bytes[i];
    }

    vec
}


#[macro_export]
macro_rules! embed {
    ($asset:expr) => {{
        let bytes_arr = include_bytes!(asset);
        __arr_to_vec(&bytes_arr[..])
    }}
}
