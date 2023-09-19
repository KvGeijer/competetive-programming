use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    let vaccines_per_day: Vec<_> = line
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    let infront_of_friends: Vec<_> = line
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // Vet hur många framför
    // Vill veta hur lång tid der tar givet ett visst antal personer

    let mut day_sums = vec![0];
    for nbr in vaccines_per_day {
        let new = *day_sums.last().unwrap() + nbr;
        day_sums.push(new);
    }

    for infront in infront_of_friends {
        let pos = infront + 1;
        let mut low = 1;
        let mut high = day_sums.len(); // One above possible

        // Vill hitta dagen då det är precis mer än infront på day_sums
        while low < high {
            let mid = (low + high) / 2;
            if day_sums[mid] >= pos {
                // På denna dag är pos processed, så en ok dag
                high = mid;
            } else if day_sums[mid] < pos {
                // Inte ännu processed, så större!
                low = mid + 1;
            }
        }

        if infront > *day_sums.last().unwrap() {
            println!("-1");
        } else {
            println!("{}", low);
        }
    }
}
