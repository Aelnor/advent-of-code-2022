use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn move_down(&mut self, cave_state: &HashSet<Point>) -> bool {
        if !cave_state.contains(&Point {
            x: self.x,
            y: self.y + 1,
        }) {
            self.y += 1;
            return true;
        }
        if !cave_state.contains(&Point {
            x: self.x - 1,
            y: self.y + 1,
        }) {
            self.y += 1;
            self.x -= 1;
            return true;
        }

        if !cave_state.contains(&Point {
            x: self.x + 1,
            y: self.y + 1,
        }) {
            self.y += 1;
            self.x += 1;
            return true;
        }
        return false;
    }
}

fn part_one(cave_state: &HashSet<Point>, max_y: u32) -> u32 {
    let mut sand_count = 0;
    let mut done = false;
    let mut state = HashSet::new();
    state.clone_from(cave_state);
    loop {
        let mut sand = Point { x: 500, y: 0 };

        loop {
            if sand.y == max_y {
                done = true;
                break;
            }

            if sand.move_down(&mut state) {
                continue;
            }

            state.insert(sand);
            break;
        }
        if done {
            return sand_count;
        }
        sand_count += 1;
    }
}

fn part_two(cave_state: &HashSet<Point>, max_y: u32) -> u32 {
    let mut sand_count = 0;
    let mut state = HashSet::new();
    state.clone_from(cave_state);
    loop {
        let mut sand = Point { x: 500, y: 0 };

        loop {
            if sand.y == max_y + 1 {
                state.insert(sand);
                break;
            }

            if sand.move_down(&mut state) {
                continue;
            }
            state.insert(sand);
            break;
        }
        sand_count += 1;
        if sand.y == 0 {
            return sand_count;
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut cave_state = HashSet::new();
    let mut max_y = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let points = line.split(" -> ").collect::<Vec<&str>>();

        let mut prev_x = 0;
        let mut prev_y = 0;

        for point in points {
            let coords = point.split(",").collect::<Vec<&str>>();
            let x = coords[0].parse::<u32>().unwrap();
            let y = coords[1].parse::<u32>().unwrap();

            if prev_x != 0 {
                if prev_x == x {
                    for i in std::cmp::min(prev_y, y)..=std::cmp::max(prev_y, y) {
                        cave_state.insert(Point { x, y: i });
                    }
                } else {
                    for i in std::cmp::min(prev_x, x)..=std::cmp::max(prev_x, x) {
                        cave_state.insert(Point { x: i, y });
                    }
                }
            }

            prev_x = x;
            prev_y = y;
            max_y = std::cmp::max(max_y, y);
        }
    }

    println!("part_one: {}", part_one(&cave_state, max_y));
    println!("part_one: {}", part_two(&cave_state, max_y));
}
