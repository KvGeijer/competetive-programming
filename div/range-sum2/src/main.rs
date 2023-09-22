use itertools::Itertools;
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");

    let vec1: Vec<_> = line
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let (_n, q) = (vec1[0], vec1[1]);

    let mut prefixes = vec![0];
    line.clear();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    let numbers = line.trim().split(" ").map(|s| s.parse::<usize>().unwrap());
    for nbr in numbers {
        prefixes.push(*prefixes.last().unwrap() + nbr);
    }

    for _ in 0..q {
        line.clear();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read from stdin");
        let (op, f, t) = line
            .trim()
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        match op {
            1 => {
                let sum = prefixes[t + 1] - prefixes[f];
                println!("{sum}");
            } 
            2 => 
            x => panic!("{x}"),
        }

    }
}
