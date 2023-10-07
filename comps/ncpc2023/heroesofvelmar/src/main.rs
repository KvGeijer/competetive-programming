#[allow(unused_imports)]
use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

// Läste fel på parsing först, så lite kaos här
fn parse(lines: &mut impl Iterator<Item = String>) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut ans1 = vec![];
    let mut ans2 = vec![];
    for _ in 0..3 {
        let line = lines.next().unwrap().to_string();
        let words = line.split_whitespace();

        let mut cards1 = vec![];
        for card in words.skip(1) {
            cards1.push(card.to_string());
        }
        ans1.push(cards1);

        let line = lines.next().unwrap().to_string();
        let words = line.split_whitespace();

        let mut cards2 = vec![];
        for card in words.skip(1) {
            cards2.push(card.to_string());
        }
        ans2.push(cards2);
    }
    (ans1, ans2)
}

fn base(card: &str) -> usize {
    match card {
        "Shadow" => 6,
        "Gale" => 5,
        "Ranger" => 4,
        "Anvil" => 7,
        "Vexia" => 3,
        "Guardian" => 8,
        "Thunderheart" => 6,
        "Frostwhisper" => 2,
        "Voidclaw" => 3,
        "Ironwood" => 3,
        "Zenith" => 4,
        "Seraphina" => 1,
        other => panic!("Card {other} encountered!"),
    }
}

fn power(cards: Vec<Vec<String>>) -> Vec<usize> {
    let mut powers = vec![];
    for (loc_ind, loc) in cards.iter().enumerate() {
        let mut power = 0;
        for card in loc.iter() {
            power += base(card);
            if card == "Thunderheart" && loc.len() == 4 {
                power += 6;
            } else if card == "Zenith" && loc_ind == 1 {
                power += 5;
            } else if card == "Seraphina" {
                power += loc.len() - 1;
            }
        }
        powers.push(power)
    }
    powers
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    // let cards1 = parse(lines);
    // let cards2 = parse(lines);
    let (cards1, cards2) = parse(lines);

    let powers1 = power(cards1);
    let powers2 = power(cards2);

    let mut diffs: Vec<i64> = vec![];
    for (p1, p2) in powers1.iter().zip(powers2.iter()) {
        diffs.push(*p1 as i64 - *p2 as i64);
    }

    // println!("p1 {:?}", powers1);
    // println!("p2 {:?}", powers2);
    // println!("diffs {:?}", diffs);

    let mut win_diff: i64 = 0;
    for diff in diffs.iter() {
        if *diff > 0 {
            win_diff += 1;
        } else if *diff < 0 {
            win_diff -= 1;
        }
    }

    if win_diff > 0 {
        println!("Player 1")
    } else if win_diff < 0 {
        println!("Player 2")
    } else {
        let sum: i64 = diffs.into_iter().sum();
        if sum > 0 {
            println!("Player 1")
        } else if sum < 0 {
            println!("Player 2")
        } else {
            println!("Tie")
        }
    }
}
