extern crate comp_template;

#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let line = lines.next().unwrap();

    let mut white = 0;
    let mut lower = 0;
    let mut upper = 0;
    let mut symbs = 0;

    for char in line.trim().chars() {
        if char == '_' {
            white += 1;
        } else if char.is_lowercase() {
            lower += 1;
        } else if char.is_uppercase() {
            upper += 1;
        } else {
            symbs += 1;
        }
    }

    println!("{}", white as f64 / line.trim().len() as f64);
    println!("{}", lower as f64 / line.trim().len() as f64);
    println!("{}", upper as f64 / line.trim().len() as f64);
    println!("{}", symbs as f64 / line.trim().len() as f64);
}
