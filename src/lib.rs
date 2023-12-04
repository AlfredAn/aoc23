use std::io::Read;

pub fn read_stdin_to_bytes() -> Vec<u8> {
    let mut buf = Vec::new();
    std::io::stdin().lock().read_to_end(&mut buf).unwrap();
    buf
}

pub fn read_stdin_to_string() -> String {
    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    buf
}
