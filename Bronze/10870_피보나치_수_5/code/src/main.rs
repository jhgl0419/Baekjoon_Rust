// 10870번: 피보나치 수 5
// Bronze 2
// 메모리: , 시간: 

use std::io::{self, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse::<usize>().unwrap();

    match n {
        0 | 1 => writeln!(out, "{}", n).unwrap(),
        _ => {
            let mut fibo: [usize; 21] = [0; 21];
            fibo[1] = 1;

            for i in 2..=n {
                fibo[i] = fibo[i-1] + fibo[i-2];
            }

            writeln!(out, "{}", fibo[n]).unwrap();
        }
    }
}
