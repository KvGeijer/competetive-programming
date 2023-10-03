#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let days = lines.read_uint();
    let costs = lines.read_uint_lines(days);

    let max_shares = 100000;
    let mut bank = 100;
    let mut shares = 0;

    for (&today, &tomorrow) in costs.clone().iter().zip(costs[1..].iter()) {
        if today < tomorrow {
            // Buy as much as you can
            let left = max_shares - shares;
            let can_buy = (bank / today).min(left);

            bank -= can_buy * today;
            shares += can_buy;
            assert!(shares <= max_shares);
        } else {
            // Sell everything
            bank += shares * today;
            shares = 0;
        }
    }

    bank += shares * costs[costs.len() - 1];
    println!("{bank}");
}
