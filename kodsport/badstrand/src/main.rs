use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    // let mut lines = locked.lines();

    let budget = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(" ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let costs: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(" ")
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    // let mut prefixes = vec![];
    // for cost in costs {
    //     let new = *prefixes.last().unwrap_or(&0) + cost;
    //     prefixes.push(new);
    // }

    let mut lower = 0;

    let mut best = 0;
    let mut sum = 0;

    // Find best range for the upper limit
    for upper in 0..costs.len() {
        sum += costs[upper];
        // Increase lower until we are in budget
        while sum > budget {
            sum -= costs[lower];
            lower += 1;
        }

        // Now best possible range stopping at upper
        let len = upper - lower + 1;
        if len > best {
            best = len
        }
    }

    println!("{best}");
}
