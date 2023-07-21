#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

#[allow(unused_macros)]
macro_rules! write_ln {
    ($input: expr) => {{
        let out = &mut BufWriter::new(stdout());
        writeln!(out, "{}", $input).ok();
    }};
}

#[allow(unused_macros)]
macro_rules! write_out {
    ($input: expr) => {{
        let out = &mut BufWriter::new(stdout());
        write!(out, "{}", $input).ok();
    }};
}
fn main() {
    let mut scan = Scanner::default();
    // let n: i64 = scan.next::<i64>();
}
