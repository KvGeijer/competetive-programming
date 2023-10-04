#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

// fn solve(lines: &mut impl Iterator<Item = String>) {
//     let (names, prizes) = lines.read_2uints();

//     // Dynamic programming.
//     // Three main variables:
//     // Prob of not drawing any of your names (easy)
//     // Prob of drawing at least one of your names (easy, lin time)
//     // Prob of drawing at least two of your names (how?)

//     // Draw prizes times
// }

fn solve(lines: &mut impl Iterator<Item = String>) {
    let (names, prizes) = lines.read_2uints();

    let mut last = 0.0;
    // let mut ps = vec![];
    for added in 1..100 * names {
        let tot_names = names + added;
        let p = prob(prizes, added, tot_names);
        if p < last {
            println!("{last}");
            return;
        }
        last = p;
        // ps.push(p);
    }
    panic!("Nooo")

    // let res = ps.into_iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    // println!("{res}");
}

fn prob(draws: usize, your_items: usize, tot_items: usize) -> f64 {
    let mut all_miss = 1.0;
    let mut one_hit = 0.0;

    for drawn in 0..draws {
        // Chance of hitting depending on how many hits/misses before
        let first_hit_prob = your_items as f64 / (tot_items - drawn) as f64;
        let first_miss_prob = 1.0 - first_hit_prob;
        let sec_hit_prob = (your_items - 1) as f64 / (tot_items - drawn) as f64;
        let sec_miss_prob = 1.0 - sec_hit_prob;

        // Importnant to update in correct order
        one_hit = all_miss * first_hit_prob + sec_miss_prob * one_hit;
        all_miss *= first_miss_prob;
    }

    one_hit
}
