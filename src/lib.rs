use std::{fmt, io::Read};

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

struct Display<F>(F);

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Debug for Display<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0(f)
    }
}

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Display for Display<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0(f)
    }
}

pub fn display<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result>(
    f: F,
) -> impl fmt::Debug + fmt::Display {
    Display(f)
}
