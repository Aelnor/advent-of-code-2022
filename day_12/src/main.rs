use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_reachable(
    terrain: &Vec<Vec<char>>,
    cur_x: usize,
    cur_y: usize,
    target_x: usize,
    target_y: usize,
) -> bool {
    let current_height = terrain[cur_y][cur_x] as u32;
    let target_height = terrain[target_y][target_x] as u32;

    current_height + 1 >= target_height
}

fn traverse(
    terrain: &Vec<Vec<char>>,
    distance_map: &mut Vec<Vec<u32>>,
    cur_x: usize,
    cur_y: usize,
    target_x: usize,
    target_y: usize,
    minimum_steps: &mut u32,
) {
    let current_steps = distance_map[cur_y][cur_x];
    if cur_x == target_x && cur_y == target_y {
        *minimum_steps = std::cmp::min(*minimum_steps, current_steps);
        return;
    }

    if cur_y < terrain.len() - 1 && is_reachable(terrain, cur_x, cur_y, cur_x, cur_y + 1) {
        let neighbor_steps = distance_map[cur_y + 1][cur_x];
        if neighbor_steps > current_steps + 1 {
            distance_map[cur_y + 1][cur_x] = current_steps + 1;
            traverse(
                terrain,
                distance_map,
                cur_x,
                cur_y + 1,
                target_x,
                target_y,
                minimum_steps,
            )
        }
    }

    if cur_y != 0 && is_reachable(terrain, cur_x, cur_y, cur_x, cur_y - 1) {
        let neighbor_steps = distance_map[cur_y - 1][cur_x];
        if neighbor_steps > current_steps + 1 {
            distance_map[cur_y - 1][cur_x] = current_steps + 1;
            traverse(
                terrain,
                distance_map,
                cur_x,
                cur_y - 1,
                target_x,
                target_y,
                minimum_steps,
            )
        }
    }

    if cur_x < terrain[0].len() - 1 && is_reachable(terrain, cur_x, cur_y, cur_x + 1, cur_y) {
        let neighbor_steps = distance_map[cur_y][cur_x + 1];
        if neighbor_steps > current_steps + 1 {
            distance_map[cur_y][cur_x + 1] = current_steps + 1;
            traverse(
                terrain,
                distance_map,
                cur_x + 1,
                cur_y,
                target_x,
                target_y,
                minimum_steps,
            )
        }
    }

    if cur_x != 0 && is_reachable(terrain, cur_x, cur_y, cur_x - 1, cur_y) {
        let neighbor_steps = distance_map[cur_y][cur_x - 1];
        if neighbor_steps > current_steps + 1 {
            distance_map[cur_y][cur_x - 1] = current_steps + 1;
            traverse(
                terrain,
                distance_map,
                cur_x - 1,
                cur_y,
                target_x,
                target_y,
                minimum_steps,
            )
        }
    }
}

fn traverse_back(
    terrain: &Vec<Vec<char>>,
    distance_map: &mut Vec<Vec<u32>>,
    cur_x: usize,
    cur_y: usize,
    minimum_steps: &mut u32,
) {
    let current_steps = distance_map[cur_y][cur_x];
    if terrain[cur_y][cur_x] == 'a' {
        *minimum_steps = std::cmp::min(*minimum_steps, current_steps);
        return;
    }

    if cur_y < terrain.len() - 1 && is_reachable(terrain, cur_x, cur_y + 1, cur_x, cur_y) {
        let neighbor_steps = distance_map[cur_y + 1][cur_x];
        if neighbor_steps > current_steps + 1 {
            distance_map[cur_y + 1][cur_x] = current_steps + 1;
            traverse_back(terrain, distance_map, cur_x, cur_y + 1, minimum_steps)
        }
    }

    if cur_y != 0 && is_reachable(terrain, cur_x, cur_y - 1, cur_x, cur_y) {
        let neighbor_steps = distance_map[cur_y - 1][cur_x];
        if neighbor_steps > current_steps + 1 {
            distance_map[cur_y - 1][cur_x] = current_steps + 1;
            traverse_back(terrain, distance_map, cur_x, cur_y - 1, minimum_steps)
        }
    }

    if cur_x < terrain[0].len() - 1 && is_reachable(terrain, cur_x + 1, cur_y, cur_x, cur_y) {
        let neighbor_steps = distance_map[cur_y][cur_x + 1];
        if neighbor_steps > current_steps + 1 {
            distance_map[cur_y][cur_x + 1] = current_steps + 1;
            traverse_back(terrain, distance_map, cur_x + 1, cur_y, minimum_steps)
        }
    }

    if cur_x != 0 && is_reachable(terrain, cur_x - 1, cur_y, cur_x, cur_y) {
        let neighbor_steps = distance_map[cur_y][cur_x - 1];
        if neighbor_steps > current_steps + 1 {
            distance_map[cur_y][cur_x - 1] = current_steps + 1;
            traverse_back(terrain, distance_map, cur_x - 1, cur_y, minimum_steps)
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut terrain = Vec::new();
    let mut distance_map = Vec::new();

    for line in reader.lines() {
        let v: Vec<char> = line.unwrap().chars().collect();
        distance_map.push(vec![std::u32::MAX; v.len()]);
        terrain.push(v);
    }

    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut target_x = 0;
    let mut target_y = 0;

    for i in 0..terrain.len() {
        for j in 0..terrain[0].len() {
            if terrain[i][j] == 'S' {
                cur_x = j;
                cur_y = i;
                terrain[i][j] = 'a';
                continue;
            }
            if terrain[i][j] == 'E' {
                target_x = j;
                target_y = i;
                terrain[i][j] = 'z';
                continue;
            }
        }
    }

    let mut minimum_steps = std::u32::MAX;
    distance_map[cur_y][cur_x] = 0;
    traverse(
        &terrain,
        &mut distance_map,
        cur_x,
        cur_y,
        target_x,
        target_y,
        &mut minimum_steps,
    );

    println!("min steps: {}", minimum_steps);
    minimum_steps = std::u32::MAX;

    for i in 0..distance_map.len() {
        for j in 0..distance_map[0].len() {
            distance_map[i][j] = std::u32::MAX;
        }
    }
    cur_x = target_x;
    cur_y = target_y;
    distance_map[cur_y][cur_x] = 0;
    traverse_back(
        &terrain,
        &mut distance_map,
        cur_x,
        cur_y,
        &mut minimum_steps,
    );
    println!("min steps: {}", minimum_steps);
}
