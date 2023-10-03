#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

struct Rank {
    stars: usize,
    rank: usize,
}

impl Rank {
    fn gain_star(&mut self) {
        if self.rank == 0 {
            return;
        }
        let max_stars = rank_stars(self.rank);
        if self.stars == max_stars {
            // Rank up
            if self.rank > 0 {
                self.rank -= 1;
                self.stars = 1;
            }
        } else {
            self.stars += 1;
        }
    }

    fn lose_star(&mut self) {
        if self.rank == 0 {
            return;
        }
        if self.rank > 20 {
            return;
        }

        if self.stars == 0 {
            // Rank down
            if self.rank > 0 && self.rank < 20 {
                self.rank += 1;
                self.stars = rank_stars(self.rank) - 1;
            }
        } else {
            self.stars -= 1;
        }
    }
}

fn rank_stars(rank: usize) -> usize {
    if rank >= 21 {
        2
    } else if rank >= 16 {
        3
    } else if rank >= 11 {
        4
    } else {
        5
    }
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let matches = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        // .inspect(|c| println!("{c}"))
        .map(|c| c == 'W')
        // .inspect(|c| println!("{c}"))
        .collect::<Vec<bool>>();

    let mut rank = Rank { stars: 0, rank: 25 };

    let mut streak = 0;
    for res in matches {
        // loop {
        // let res = lines.next().unwrap().trim() == "W";
        if res {
            streak += 1;
            if streak >= 3 && rank.rank > 5 {
                rank.gain_star();
                rank.gain_star();
            } else {
                rank.gain_star();
            }
        } else {
            streak = 0;
            rank.lose_star()
        }
        if rank.rank == 0 {
            println!("Legend");
            return;
        }
        // println!(
        //     "Rank: {}, stars: {} with res {}",
        //     rank.rank, rank.stars, res
        // );
    }

    if rank.rank == 0 {
        panic!("Lol");
        // println!("Legend");
    } else {
        println!("{}", rank.rank);
    }
}
