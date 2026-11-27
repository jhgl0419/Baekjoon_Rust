// 4344번: 평균은 넘겠지
// Bronze 1
// 메모리: 3264KB, 시간: 2ms

use std::io::{self, Read, Write};
fn main() {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut buf = String::new();

    io::stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace().map(|x: &str| x.parse::<usize>().unwrap());

    for _ in 0..input.next().unwrap() {
        let mut average: f64 = 0.0;
        let n_student = input.next().unwrap();
        let mut score: Vec<usize> = vec![0; n_student];
        for s in score.iter_mut() {
            *s = input.next().unwrap();
            average += *s as f64;
        }
        average /= n_student as f64;

        let mut percent: f64 = 0.0;
        for s in score.iter() {
            if *s as f64 > average {
                percent += 1.0;
            }
        }
        percent /= n_student as f64;

        writeln!(out, "{:.3}%", percent*100.0).unwrap();
    }
}