#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    // let (n1, n2) = lines.read_2ints();
    let n1 = lines.read_int();
    let n2 = lines.read_int();

    let diff = (n1 - n2).rem_euclid(360); // [0, 359]
    let len = if diff > 180 { 360 - diff } else { diff };

    if ((n1 + len) % 360) == n2 {
        println!("{len}")
    } else {
        println!("{}", -len)
    }
}
