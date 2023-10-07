#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let (_n, m) = lines.read_2uints();

    let mut scouts = vec![0; m];
    let mut items = lines.read_uints();

    items.sort();
    items.reverse();
    // println!("{:?}", items);
    for (i, weight) in items.into_iter().enumerate() {
        let ind = if i >= scouts.len() {
            scouts.len() - (i - scouts.len() + 1)
        } else {
            i
        };

        scouts[ind] += weight;
    }

    // println!("{:?}", scouts);
    let max = scouts.into_iter().max().unwrap();
    println!("{max}")
}
