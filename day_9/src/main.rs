use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn is_adjacent(head: &Point, tail: &Point) -> bool {
    (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1
}

fn move_tail(head: &Point, tail: &mut Point) {
    if is_adjacent(head, tail) {
        return;
    }

    if head.y > tail.y {
        tail.y += 1;
    }
    if head.y < tail.y {
        tail.y -= 1;
    }

    if head.x > tail.x {
        tail.x += 1;
    }
    if head.x < tail.x {
        tail.x -= 1;
    }
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let number_of_knots = 10;
    let mut knots = Vec::new();
    for _ in 0..number_of_knots {
        knots.push(Point { x: 0, y: 0 });
    }

    let mut visited = HashSet::new();
    visited.insert(Point { x: 0, y: 0 });
    for line in reader.lines() {
        let line = line.unwrap();
        let command: Vec<&str> = line.split(" ").collect();
        let times = command[1].parse().unwrap();

        for _ in 0..times {
            match command[0] {
                "R" => knots[0].x += 1,
                "L" => knots[0].x -= 1,
                "U" => knots[0].y += 1,
                "D" => knots[0].y -= 1,
                _ => unreachable!(),
            }
            for i in 1..number_of_knots {
                let head = knots[i - 1].clone();
                move_tail(&head, &mut knots[i]);
                visited.insert(knots[number_of_knots - 1].clone());
            }
        }
    }
    println!("visited: {}", visited.len());
}
