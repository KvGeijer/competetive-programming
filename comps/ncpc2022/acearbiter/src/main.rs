extern crate comp_template;

#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let n = lines.read_uint();

    let (mut first_score, mut sec_score) = (0, 0);
    for round in 1..=n {
        if !process_round(lines, &mut first_score, &mut sec_score) {
            println!("error {round}");
            return;
        }
    }
    println!("ok");
}

fn process_round(
    lines: &mut impl Iterator<Item = String>,
    prev_first: &mut usize,
    prev_sec: &mut usize,
) -> bool {
    // TODO: Read ints independent of separators
    // TODO: Collect tuple...
    let ints = lines.map(|line| line.replace("-", " ")).read_uints();
    let (serve, rec) = (ints[0], ints[1]);

    let new_total = serve + rec;

    let (first, sec) = if ((new_total + 1) / 2) % 2 == 0 {
        // Swap from original
        (rec, serve)
    } else {
        (serve, rec)
    };

    if first < *prev_first || sec < *prev_sec {
        return false;
    }
    if first > 11 || sec > 11 {
        return false;
    }

    if first >= 11 && sec >= 11 {
        return false;
    }

    if *prev_first.max(prev_sec) >= 11 && (*prev_first != first || *prev_sec != sec) {
        return false;
    }

    *prev_first = first;
    *prev_sec = sec;

    true
}
