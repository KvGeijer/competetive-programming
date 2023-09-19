use std::io::{self, BufRead};

// Hour:Minute:Sec
// 0..24:0..60:0..60

fn inc(clock: &mut [usize; 6], pos: usize) {
    let max = if pos % 2 == 1 { 9 } else { 5 };

    clock[pos] += 1;
    if clock[pos] > max {
        clock[pos] = 0;
        inc(clock, pos - 1);
    }
}

fn cost(clock: &[usize; 6], segs: &[usize; 10]) -> usize {
    clock.iter().map(|dig| segs[*dig]).sum()
}

fn main() {
    let total = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    let segs = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

    let mut end = [0, 0, 0, 0, 0, 0];
    let mut start = [0, 0, 0, 0, 0, 0];
    let mut sum: usize = cost(&end, &segs);
    let mut nbr_solutions: usize = 0;

    // Loop until
    loop {
        // Loop while within the day.
        inc(&mut end, 5);
        if end[0] == 2 && end[1] == 4 {
            break;
        }

        // This must increase the sum
        sum += cost(&end, &segs);

        // Bring up start point until we are at or below total.
        while sum > total {
            sum -= cost(&start, &segs);
            inc(&mut start, 5);
        }

        // Did we find a solution?
        if sum == total {
            nbr_solutions += 1;
        }
    }

    println!("{nbr_solutions}");
}
