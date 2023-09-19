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
    for nbr in line.trim().split(" ").map(|s| s.parse::<usize>().unwrap()) {
        prefixes.push(*prefixes.last().unwrap() + nbr);
    }

    for _ in 0..q {
        line.clear();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read from stdin");
        let vec2: Vec<_> = line
            .trim()
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let (f, t) = (vec2[0], vec2[1]);

        let sum = prefixes[t + 1] - prefixes[f];
        println!("{sum}");
    }
}
