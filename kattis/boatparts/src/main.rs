extern crate comp_template;

#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let (p, n) = lines.read_2uints();

    let mut parts = HashSet::new();

    for day in 0..n {
        let part = lines.next().unwrap();
        parts.insert(part);
        if parts.len() >= p {
            println!("{}", day + 1);
            return;
        }
    }
    println!("paradox avoided");
}
