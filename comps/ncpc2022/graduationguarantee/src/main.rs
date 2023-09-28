extern crate comp_template;

#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let (n, k) = lines.read_2uints();

    let mut probs: Vec<_> = lines.read_floats().into_iter().map(|f| f as f64).collect();
    probs.sort_by(|x, y| x.total_cmp(y));

    // let mut cache: HashMap<(i32, usize), f64> = HashMap::new();
    let mut cache: Vec<Vec<f64>> = vec![vec![2.0; n + 1]; 2 * n + 2]; // First index: k, second: q. The hashmap solution was too slow
    let prob_pass = (0..=probs.len())
        .rev()
        .map(|q| prob(k as i32, q, &probs, &mut cache))
        .max_by(|x, y| x.total_cmp(y))
        .unwrap();

    println!("{prob_pass}");
}

// What is the probability of getting at least k right, given we have q questions left to answer?
fn prob(k: i32, q: usize, probs: &[f64], cache: &mut Vec<Vec<f64>>) -> f64 {
    if k > q as i32 {
        // Cannot be right anymore
        return 0.0;
    } else if k <= -(q as i32) {
        return 1.0;
    } else if q == 0 {
        // We have answered all questions
        if k <= 0 {
            return 1.0;
        } else {
            return 0.0;
        }
    } else if cache[(k + probs.len() as i32) as usize][q] < 1.5 {
        return cache[(k + probs.len() as i32) as usize][q];
    }
    // We either take or don't take the question.
    let prob_hit = prob(k - 1, q - 1, probs, cache);
    let prob_miss = prob(k + 1, q - 1, probs, cache);
    let prob_take = prob_hit * probs[probs.len() - q] + prob_miss * (1.0 - probs[probs.len() - q]);

    cache[(k + probs.len() as i32) as usize][q] = prob_take;
    // cache.insert((k, q), prob_take);

    prob_take
}
