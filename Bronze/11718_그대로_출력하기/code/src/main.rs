// 11718번: 그대로 출력하기
// Bronze 3
// 메모리: 3240KB, 속도: 2ms

use std::io::{self, Read, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    writeln!(out, "{}", buf).unwrap();
}
