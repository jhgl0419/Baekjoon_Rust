// 1781번: 컵라면
// Gold 2
// 메모리: 	13504KB, 시간: 22ms

use std::collections::BinaryHeap;
use std::io::{self, Read, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace().map(|x:&str| x.parse::<usize>().unwrap());

    let n: usize = input.next().unwrap();
    let mut dl_n_r: Vec<(usize, usize)> = vec![];

    for _ in 0..n {
        dl_n_r.push((input.next().unwrap(), input.next().unwrap()));
    }
    dl_n_r.sort_unstable_by_key(|a| (a.0, a.1));

    let mut candidate = BinaryHeap::<usize>::new();

    let mut last_i: usize = n-1;
    let mut sum: usize = 0;
    for t in (1..=n).rev() {
        for i in (0..=last_i).rev() {
            if dl_n_r[i].0 == t {
                candidate.push(dl_n_r[i].1);
                writeln!(out, "cur t: {}, deadline: {}, ramen: {}", t, dl_n_r[i].0, dl_n_r[i].1).unwrap();
            } else {
                last_i = i;
                break;
            }
        }

        if let Some(r) = candidate.pop() {
            writeln!(out, "cur t: {}, pop: {}", t, r).unwrap();
            sum += r;
        }
    }

    writeln!(out, "{}", sum).unwrap();
}
