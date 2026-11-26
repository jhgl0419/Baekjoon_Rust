// 5597번: 과제 안 내신 분..?
// Bronze3
// 메모리: , 속도: 

use std::io::{self, Read, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_to_string(&mut buf).unwrap();

    let mut submit: [bool; 31] = [false; 31];
    submit[0] = true;
    for n in buf.split_ascii_whitespace().map(|s: &str| s.parse::<usize>().unwrap()) {
        submit[n] = true;
    }

    for (i, b) in submit.iter().enumerate() {
        if !b {
            writeln!(out, "{}", i).unwrap();
        }
    }
}