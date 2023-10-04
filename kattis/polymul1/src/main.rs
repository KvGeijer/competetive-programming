#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let tests = lines.read_uint();

    for _ in 0..tests {
        let p1 = {
            lines.read_int();
            lines.read_ints()
        };

        let p2 = {
            lines.read_int();
            lines.read_ints()
        };

        let mut p3 = vec![0; p1.len() + p2.len() - 1];
        for i in 0..p1.len() {
            for j in 0..p2.len() {
                p3[i + j] += p1[i] * p2[j];
            }
        }

        println!("{}", p3.len() - 1);
        for item in p3 {
            print!("{item} ");
        }
        println!("")
    }
}
