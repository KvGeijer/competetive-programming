extern crate comp_template;

#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let l = io::stdin();
    let mut lines = l.lock().lines().map(|line| line.unwrap());
    // let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let m = lines.read_uint();
    let n = lines.read_uint();

    let courses = lines.read_uint_lines(n);

    // So eat something every m.
    // Dyn: How many ways can we eat with "t" min left?

    let mut cache = vec![0; m + 1];
    cache[0] = 1;
    for t in 0..=m {
        if cache[t] > 0 {
            for course in courses.iter() {
                if t + *course <= m {
                    cache[t + *course] += cache[t];
                }
            }
        }
    }

    println!("{}", cache[m])
}
