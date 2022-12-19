use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

fn part1(cubes: &HashSet<Vector3>) -> i32 {
    let mut result = 0;
    for cube in cubes {
        if !cubes.contains(&Vector3 {
            x: cube.x - 1,
            y: cube.y,
            z: cube.z,
        }) {
            result += 1;
        }

        if !cubes.contains(&Vector3 {
            x: cube.x + 1,
            y: cube.y,
            z: cube.z,
        }) {
            result += 1;
        }

        if !cubes.contains(&Vector3 {
            x: cube.x,
            y: cube.y - 1,
            z: cube.z,
        }) {
            result += 1;
        }
        if !cubes.contains(&Vector3 {
            x: cube.x,
            y: cube.y + 1,
            z: cube.z,
        }) {
            result += 1;
        }
        if !cubes.contains(&Vector3 {
            x: cube.x,
            y: cube.y,
            z: cube.z - 1,
        }) {
            result += 1;
        }
        if !cubes.contains(&Vector3 {
            x: cube.x,
            y: cube.y,
            z: cube.z + 1,
        }) {
            result += 1;
        }
    }
    result
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut cubes = HashSet::new();
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut min_z = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;
    let mut max_z = std::i32::MIN;
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(',').collect();
        let v = Vector3 {
            x: split[0].parse::<i32>().unwrap(),
            y: split[1].parse::<i32>().unwrap(),
            z: split[2].parse::<i32>().unwrap(),
        };

        min_x = std::cmp::min(min_x, v.x);
        min_y = std::cmp::min(min_y, v.y);
        min_z = std::cmp::min(min_z, v.z);

        max_x = std::cmp::max(max_x, v.x);
        max_y = std::cmp::max(max_y, v.y);
        max_z = std::cmp::max(max_z, v.z);

        cubes.insert(v);
    }

    println!("surface: {}", part1(&cubes));

    let mut air = HashSet::new();
    air.insert(Vector3 {
        x: max_x - 1,
        y: min_y - 1,
        z: min_z - 1,
    });

    let mut done = false;
    let mut processed = HashSet::new();
    while !done {
        done = true;
        let mut new_air = HashSet::new();
        for cube in &air {
            if processed.contains(cube) {
                continue;
            }
            processed.insert(cube.clone());
            if cube.x != min_x - 1 {
                let v = Vector3 {
                    x: cube.x - 1,
                    y: cube.y,
                    z: cube.z,
                };
                if !cubes.contains(&v) && !air.contains(&v) {
                    done = false;
                    new_air.insert(v);
                }
            }

            if cube.x != max_x + 1 {
                let v = Vector3 {
                    x: cube.x + 1,
                    y: cube.y,
                    z: cube.z,
                };
                if !cubes.contains(&v) && !air.contains(&v) {
                    done = false;
                    new_air.insert(v);
                }
            }

            if cube.y != min_y - 1 {
                let v = Vector3 {
                    x: cube.x,
                    y: cube.y - 1,
                    z: cube.z,
                };
                if !cubes.contains(&v) && !air.contains(&v) {
                    done = false;
                    new_air.insert(v);
                }
            }

            if cube.y != max_y + 1 {
                let v = Vector3 {
                    x: cube.x,
                    y: cube.y + 1,
                    z: cube.z,
                };
                if !cubes.contains(&v) && !air.contains(&v) {
                    done = false;
                    new_air.insert(v);
                }
            }

            if cube.z != min_z - 1 {
                let v = Vector3 {
                    x: cube.x,
                    y: cube.y,
                    z: cube.z - 1,
                };
                if !cubes.contains(&v) && !air.contains(&v) {
                    done = false;
                    new_air.insert(v);
                }
            }

            if cube.z != max_z + 1 {
                let v = Vector3 {
                    x: cube.x,
                    y: cube.y,
                    z: cube.z + 1,
                };
                if !cubes.contains(&v) && !air.contains(&v) {
                    done = false;
                    new_air.insert(v);
                }
            }
        }
        air.extend(new_air);
    }
    println!("cubes: {}", cubes.len());
    println!("air: {}", air.len());

    let mut result = 0;
    for a in &air {
        if cubes.contains(&Vector3 {
            x: a.x - 1,
            y: a.y,
            z: a.z,
        }) {
            result += 1;
        }

        if cubes.contains(&Vector3 {
            x: a.x + 1,
            y: a.y,
            z: a.z,
        }) {
            result += 1;
        }

        if cubes.contains(&Vector3 {
            x: a.x,
            y: a.y - 1,
            z: a.z,
        }) {
            result += 1;
        }
        if cubes.contains(&Vector3 {
            x: a.x,
            y: a.y + 1,
            z: a.z,
        }) {
            result += 1;
        }
        if cubes.contains(&Vector3 {
            x: a.x,
            y: a.y,
            z: a.z - 1,
        }) {
            result += 1;
        }
        if cubes.contains(&Vector3 {
            x: a.x,
            y: a.y,
            z: a.z + 1,
        }) {
            result += 1;
        }
    }
    println!("{}", result);
}
