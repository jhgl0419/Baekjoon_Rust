// 1085번: 직사각형에서 탈출
// Bronze 3
// 메모리: 3244KB, 속도: 1ms

use std::io::{self, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut v = buf.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap());

    let (x, y, w, h) = (v.next().unwrap(), v.next().unwrap(), v.next().unwrap(), v.next().unwrap());

    writeln!(out, "{}", x.min((w-x).min(y.min(h-y)))).unwrap();
}