extern crate comp_template;

use comp_template::parsing::CompIterParser;

use std::io::{self, BufRead};

fn main() {
    let mut lines: _ = io::stdin().lock().lines().map(|line| line.unwrap());
    solve(&mut lines);
}

struct Segment {
    start: usize,
    stop: usize,
    sum: i64,
    left: Option<Box<Segment>>,
    right: Option<Box<Segment>>,
}

impl Segment {
    pub fn new(arr: &[i64]) -> Segment {
        Segment::new_rec(arr, 0, arr.len())
    }

    fn new_rec(arr: &[i64], start: usize, stop: usize) -> Segment {
        if start == stop - 1 {
            Segment {
                start,
                stop,
                sum: arr[start],
                left: None,
                right: None,
            }
        } else {
            let mid = (start + stop) / 2;
            assert!(mid > start);
            assert!(mid < stop);
            let left = Segment::new_rec(arr, start, mid);
            let right = Segment::new_rec(arr, mid, stop);
            let sum = left.sum + right.sum;
            Segment {
                start,
                stop,
                sum,
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            }
        }
    }

    /// Update sums  log(n)
    fn update(&mut self, ind: usize, new: i64) -> i64 {
        if let (Some(left), Some(right)) = (&mut self.left, &mut self.right) {
            if ind < left.stop {
                // Go left
                let diff = left.update(ind, new);
                self.sum += diff;
                diff
            } else {
                // Go right
                let diff = right.update(ind, new);
                self.sum += diff;
                diff
            }
        } else {
            assert!(self.start + 1 == self.stop && self.start == ind);
            let diff = new - self.sum;
            self.sum += diff;
            diff
        }
    }

    // Find the sum of all numbers in [start, end] in log(n) time
    pub fn range_sum(&self, start: usize, end: usize) -> i64 {
        if self.left.is_none() {
            assert!(self.start == start && end == start);
            self.sum
        } else if start == 0 {
            self.prefix_sum(end + 1)
        } else {
            let total = self.prefix_sum(end + 1);
            let prefix = self.prefix_sum(start);
            // println!("range {total} - {prefix}");
            total - prefix
        }
    }

    // Exclusive stop
    fn prefix_sum(&self, stop: usize) -> i64 {
        if let (Some(left), Some(right)) = (&self.left, &self.right) {
            if left.stop < stop {
                // Include the whole left and part of right
                left.sum + right.prefix_sum(stop)
            } else {
                // Include only part of the left
                left.prefix_sum(stop)
            }
        } else {
            // Reached a leaf
            self.sum
        }
    }
}

fn solve(lines: &mut impl Iterator<Item = String>) {
    let (_n, q) = lines.read_2uints();
    let numbers = lines.read_ints();
    let queries = lines.read_int_mat(q);

    let mut seg_tree = Segment::new(&numbers);

    for query in queries {
        match &query[0..3] {
            &[1, start, end] => {
                let sum = seg_tree.range_sum(start as usize, end as usize);
                println!("{sum}");
            }
            &[2, ind, to] => {
                seg_tree.update(ind as usize, to);
            }
            _other => panic!("Matched other!"),
        }
    }
}
