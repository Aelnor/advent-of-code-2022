use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn read_input() -> HashMap<Point, Option<Point>> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut elves = HashMap::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    None,
                );
            }
        }
    }
    elves
}

fn is_point_isolated(elves: &HashMap<Point, Option<Point>>, x: i32, y: i32) -> bool {
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if elves.contains_key(&Point {
                x: x + dx,
                y: y + dy,
            }) {
                return false;
            }
        }
    }
    true
}

fn vacant(elves: &HashMap<Point, Option<Point>>, x: i32, y: i32, points: &[(i32, i32)]) -> bool {
    for p in points {
        if elves.contains_key(&Point {
            x: x + p.0,
            y: y + p.1,
        }) {
            return false;
        }
    }
    true
}

fn print_map(elves: &HashMap<Point, Option<Point>>) {
    let min_x = std::cmp::min(0, elves.keys().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x);
    let max_x = elves.keys().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let min_y = std::cmp::min(0, elves.keys().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y);
    let max_y = elves.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains_key(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    pause();
}

fn move_elves(
    elves: &HashMap<Point, Option<Point>>,
    max_rounds: usize,
) -> (HashMap<Point, Option<Point>>, usize) {
    let moves = [
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
    ];

    let mut current_round = elves.clone();
    let mut moved;

    for round in 0..max_rounds {
        moved = false;
        let mut intentions_frequency = HashMap::new();
        let mut new_round = HashMap::new();

        let positions = current_round
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<Point>>();
        for position in positions {
            // check if there are elves around
            if is_point_isolated(&current_round, position.x, position.y) {
                continue;
            }

            for i in round..round + moves.len() {
                let m = &moves[i % moves.len()];
                if vacant(&current_round, position.x, position.y, m) {
                    let p = Point {
                        x: position.x + m[1].0,
                        y: position.y + m[1].1,
                    };

                    *intentions_frequency.entry(p.clone()).or_insert(0) += 1;
                    current_round.insert(position, Some(p));
                    break;
                }
            }
        }
        for position in current_round.keys() {
            let intention = current_round.get(&position).unwrap();
            if intention.is_none() {
                new_round.insert(position.clone(), None);
                continue;
            }

            let intention = intention.unwrap();

            if *intentions_frequency.get(&intention).unwrap() == 1 {
                new_round.insert(intention, None);
                moved = true;
            } else {
                new_round.insert(position.clone(), None);
            }
        }
        if !moved {
            return (current_round, round + 1);
        }
        current_round = new_round;
    }
    (current_round, max_rounds)
}

fn main() {
    let elves = read_input();
    let (after_10_round, _) = move_elves(&elves, 10);

    let min_x = after_10_round
        .keys()
        .min_by(|a, b| a.x.cmp(&b.x))
        .unwrap()
        .x;
    let max_x = after_10_round
        .keys()
        .max_by(|a, b| a.x.cmp(&b.x))
        .unwrap()
        .x;
    let min_y = after_10_round
        .keys()
        .min_by(|a, b| a.y.cmp(&b.y))
        .unwrap()
        .y;
    let max_y = after_10_round
        .keys()
        .max_by(|a, b| a.y.cmp(&b.y))
        .unwrap()
        .y;

    let mut result = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !after_10_round.contains_key(&Point { x, y }) {
                result += 1;
            }
        }
    }

    println!("result: {}", result);

    let (_, rounds) = move_elves(&elves, std::usize::MAX);
    println!("rounds: {}", rounds);
}
