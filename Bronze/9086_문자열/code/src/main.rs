// 9086번: 문자열
// Bronze 5
// 메모리: 3244KB, 속도: 1ms

use std::io::{self, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    
    let n: usize = buf.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let s: &str = buf.trim();

        writeln!(out, "{}{}", s.chars().next().unwrap(), s.chars().last().unwrap()).unwrap();
    }
}