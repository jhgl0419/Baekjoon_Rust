// 1781번: 컵라면
// Gold 2
// 메모리: 	13504KB, 시간: 22m

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

    let mut sum: usize = 0;
    for t in (1..=n).rev() {
        while let Some(&(dead, ramen)) = dl_n_r.last() {
            if dead >= t {
                candidate.push(ramen);
                dl_n_r.pop();
            } else {
                break;
            }
        }

        if let Some(r) = candidate.pop() {
            sum += r;
        }
    }

    writeln!(out, "{}", sum).unwrap();
}
