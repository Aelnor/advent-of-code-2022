use std::fs::File;
use std::io::{BufRead, BufReader};

struct Pair {
    start: u32,
    end: u32,
}

impl Pair {
    fn from(s: &str) -> Self {
        let split = s
            .split('-')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Pair {
            start: split[0],
            end: split[1],
        }
    }

    fn includes(&self, other: &Pair) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Pair) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (other.start >= self.start && other.start <= self.end)
    }
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut fully_overlap = 0;
    let mut partially_overlap = 0;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let split = line.split(',').collect::<Vec<&str>>();
        let first = Pair::from(split[0]);
        let second = Pair::from(split[1]);
        if first.includes(&second) || second.includes(&first) {
            fully_overlap += 1;
        }

        if first.overlaps(&second) {
            partially_overlap += 1;
        }
    }
    println!("Fully overlap: {}", fully_overlap);
    println!("Partially overlap: {}", partially_overlap);
}
