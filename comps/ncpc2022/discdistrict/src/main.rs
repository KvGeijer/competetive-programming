extern crate comp_template;

#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let r = lines.read_uint();

    let mut x = r + 1;
    let mut y = 0;

    let mut best_dist_sq = (r + 1) * (r + 1);
    let mut best = (r + 1, 0);

    while y <= r {
        let dist_sq = x * x + y * y;
        if dist_sq < best_dist_sq {
            best_dist_sq = dist_sq;
            best = (x, y);
        }

        // Move somewhere
        let move_left_dist = (x - 1) * (x - 1) + y * y;
        if move_left_dist > r * r {
            x -= 1;
        } else {
            y += 1;
        }
    }

    println!("{} {}", best.0, best.1);
}
