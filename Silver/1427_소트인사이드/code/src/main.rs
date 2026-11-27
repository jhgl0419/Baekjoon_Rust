// 1427번: 소트인사이드
// Silver 5
// 메모리: 3248KB, 시간: 2ms

use std::io::{self, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut num = buf.trim().chars().collect::<Vec<char>>();    // -> buf.trim().as_bytes()로 u8 vec으로 만들어서 비교하는 게 효율적
    num.sort_by(|a, b| b.cmp(a));

    writeln!(out, "{}", num.iter().collect::<String>()).unwrap();
}
