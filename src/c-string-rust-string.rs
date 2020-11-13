// some C strings are valid Rust string but reverse is never true
fn main() {
    let bytes = Box::new("Hello\0".to_owned());

    let c_str: *const u8 = bytes.as_ptr();
    let length = {
        let mut i = 0;
        loop {
            let b = unsafe { *c_str.offset(i) };
            if b == 0 {
                break i;
            }
            i += 1;
        }
    };
    let u8_slice = unsafe { std::slice::from_raw_parts(c_str, length as usize) };
    let rust_string = std::str::from_utf8(u8_slice).unwrap();

    println!("before drop = {:#?}", rust_string);
    drop(bytes);
    println!(" after drop = {:#?}", rust_string);
}
