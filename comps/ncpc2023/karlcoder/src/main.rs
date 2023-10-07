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

fn read(
    lines: &mut impl Iterator<Item = String>,
    pos: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(val) = cache.get(&pos) {
        *val
    } else {
        println!("buf[{pos}]");
        let val = lines.read_uint();
        cache.insert(pos, val);
        val
    }
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let mut cache = HashMap::new();
    let mut upper = find_upper(lines, &mut cache);

    let mut lower = upper / 2;

    while upper > lower {
        let mid = (upper + lower) / 2;
        let val = read(lines, mid, &mut cache);
        if val == 0 {
            upper = mid - 1;
            if upper < lower {
                upper = lower;
            }
        } else {
            lower = mid + 1;
            if lower > upper {
                lower = upper;
            }
        }
    }

    // println!("{upper}, {lower}");
    // assert!(upper == lower);
    if read(lines, lower, &mut cache) == 0 {
        println!("strlen(buf) = {lower}");
    } else {
        println!("strlen(buf) = {}", lower + 1);
    }
}

fn find_upper(
    lines: &mut impl Iterator<Item = String>,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    let mut shift = 1;
    loop {
        let pos = 1 << shift;
        if read(lines, pos, cache) == 0 {
            return pos;
        }
        shift += 1;
    }
}
