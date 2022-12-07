use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn vec_to_path(pwd: &Vec<String>) -> String {
    String::from(format!("/{}", pwd.join("/")))
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut cur_dir = Vec::new();

    let mut map = HashMap::new();
    map.insert(String::from("/"), 0);

    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(' ').collect();

        if split[0] == "$" {
            if split[1] == "ls" {
                continue;
            }

            if split[2] == "/" {
                cur_dir.clear();
                continue;
            }

            if split[2] == ".." {
                cur_dir.pop();
                continue;
            }

            cur_dir.push(String::from(split[2]));
            map.insert(vec_to_path(&cur_dir), 0);
            continue;
        }

        if split[0] == "dir" {
            continue;
        }

        let size = split[0].parse::<i32>().unwrap();

        let mut vcopy = cur_dir.clone();
        loop {
            let path = vec_to_path(&vcopy);
            *map.get_mut(&path).unwrap() += size;
            if vcopy.is_empty() {
                break;
            }
            vcopy.pop();
        }
    }

    let mut result = 0;
    let total_occupied = *map.get(&String::from("/")).unwrap();
    let to_free = 30000000 - (70000000 - total_occupied);
    let mut min_total_size = total_occupied;

    map.values().for_each(|v| {
        if *v < 100000 {
            result += *v;
        }

        if *v > to_free {
            min_total_size = std::cmp::min(min_total_size, *v);
        }
    });

    println!("{}", result);
    println!("{}", min_total_size);
}
