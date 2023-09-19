use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();

    // This parsing is horrible... I miss itertools... collect_tuple
    let first = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(" ")
        .map(|nbr| nbr.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (n, m) = (first[0], first[1]);

    let mut her_cards = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(" ")
        .map(|nbr| nbr.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    her_cards.sort();

    let mut my_cards = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(" ")
        .map(|nbr| nbr.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    my_cards.sort();

    if n == m {
        // Should work
        let diffs: i64 = my_cards
            .into_iter()
            .zip(her_cards.into_iter())
            .map(|(x, y)| (x - y).abs())
            .sum();
        println!("{diffs}");
    } else {
        // Dynamic prog with a cache of 2, as there is always 1 card skipped at some point
        // Best scores up til now depending on if we have or have not already skipped a card
        let mut last_skipped = 0;
        let mut last_unskipped = 0;

        // What do we play for each of her cards?
        for (ind, her_card) in her_cards.into_iter().enumerate() {
            let skipped_diff = (my_cards[ind + 1] - her_card).abs();
            let noskipped_diff = (my_cards[ind] - her_card).abs();

            last_skipped = last_skipped.min(last_unskipped) + skipped_diff;
            last_unskipped += noskipped_diff;
        }

        let best = last_skipped.min(last_unskipped);
        println!("{best}");
    }
}
