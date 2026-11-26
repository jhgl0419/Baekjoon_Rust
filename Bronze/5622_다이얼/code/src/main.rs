// 5622번: 다이얼
// Bronze 2
// 메모리: 3240KB, 속도: 2ms 

use std::io::{self, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut time: usize = 0;
    for c in buf.trim().chars() {
        match c {
            'A' | 'B' | 'C' => {time += 3;},
            'D' | 'E' | 'F' => {time += 4;},
            'G' | 'H' | 'I' => {time += 5;},
            'J' | 'K' | 'L' => {time += 6;},
            'M' | 'N' | 'O' => {time += 7;},
            'P' | 'Q' | 'R' | 'S' => {time += 8;},
            'T' | 'U' | 'V' => {time += 9;},
            'W' | 'X' | 'Y' => {time += 10;},
            _ => {continue;},
        }
    }
    writeln!(out, "{}", time).unwrap();
}