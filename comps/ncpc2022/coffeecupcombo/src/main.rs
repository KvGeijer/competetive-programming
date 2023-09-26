extern crate comp_template;

#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().trim().to_string());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let _n = lines.read_uint();
    let machines = lines.next().unwrap();

    let mut left: i64 = 0;
    let mut awake = 0;
    for machine in machines.chars() {
        if machine == '1' || left > 0 {
            awake += 1;
        }

        if left > 0 {
            left -= 1;
        }

        if machine == '1' {
            left = 2;
        }
    }

    println!("{awake}");
}
