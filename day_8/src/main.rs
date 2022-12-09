use std::fs::File;
use std::io::{BufRead, BufReader};

fn tree_visible(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let height = forest[y][x];

    let mut found_higher = false;
    for i in 0..y {
        if forest[i][x] >= height {
            found_higher = true;
            break;
        }
    }

    if !found_higher {
        return true;
    }

    found_higher = false;
    for i in y + 1..forest.len() {
        if forest[i][x] >= height {
            found_higher = true;
            break;
        }
    }

    if !found_higher {
        return true;
    }

    found_higher = false;
    for i in 0..x {
        if forest[y][i] >= height {
            found_higher = true;
            break;
        }
    }

    if !found_higher {
        return true;
    }

    found_higher = false;
    for i in x + 1..forest[0].len() {
        if forest[y][i] >= height {
            found_higher = true;
            break;
        }
    }

    !found_higher
}

fn get_scenic_score(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let height = forest[y][x];
    let mut scenic_score = 1;

    let mut trees_seen = 0;
    for i in x + 1..forest[0].len() {
        trees_seen += 1;
        if forest[y][i] >= height {
            break;
        }
    }
    scenic_score *= trees_seen;

    trees_seen = 0;
    for i in y + 1..forest.len() {
        trees_seen += 1;
        if forest[i][x] >= height {
            break;
        }
    }
    scenic_score *= trees_seen;

    trees_seen = 0;
    if y != 0 {
        for i in (0..y).rev() {
            trees_seen += 1;
            if forest[i][x] >= height {
                break;
            }
        }
    }
    scenic_score *= trees_seen;

    trees_seen = 0;
    if x != 0 {
        for i in (0..x).rev() {
            trees_seen += 1;
            if forest[y][i] >= height {
                break;
            }
        }
    }
    scenic_score *= trees_seen;

    scenic_score
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut forest_map = Vec::new();
    for line in reader.lines() {
        let trees_line: Vec<u32> = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        forest_map.push(trees_line);
    }

    let mut result = 0;
    let mut max_scenic_score = 0 as u32;

    for i in 0..forest_map.len() {
        for j in 0..forest_map[0].len() {
            if tree_visible(&forest_map, j, i) {
                result += 1;
            }

            max_scenic_score = std::cmp::max(max_scenic_score, get_scenic_score(&forest_map, j, i));
        }
    }
    println!("{}", result);
    println!("Max Scenic Score: {}", max_scenic_score);
}
