use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        return item as u32 - 'A' as u32 + 27;
    }
    if item.is_ascii_lowercase() {
        return item as u32 - 'a' as u32 + 1;
    }

    unreachable!()
}

fn find_common_item(rucksacks: &[String]) -> char {
    assert_eq!(rucksacks.len(), 3);

    let mut item_map = HashMap::new();

    rucksacks.into_iter().for_each(|rucksack| {
        let r: HashSet<_> = rucksack.chars().collect();
        r.into_iter()
            .for_each(|item| *item_map.entry(item).or_insert(0) += 1)
    });

    for (v, k) in item_map {
        if k == 3 {
            return v;
        }
    }

    unreachable!();
}

fn part_one() {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;

    for (_, line) in reader.lines().enumerate() {
        let mut found = false;
        let line = line.unwrap();

        let mut contents = HashMap::new();
        for i in 0..line.len() / 2 {
            contents.insert(line.chars().nth(i).unwrap(), 0);
        }

        for i in line.len() / 2..line.len() {
            let item = &line.chars().nth(i).unwrap();
            if contents.contains_key(&item) {
                found = true;
                sum += get_priority(*item);
                break;
            }
        }
        assert!(found)
    }
    println!("sum of priorities: {}", sum);
}

fn part_two() {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;

    let mut v = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        v.push(line.unwrap());
        if v.len() == 3 {
            let common_item = find_common_item(&v);
            sum += get_priority(common_item);
            v.clear();
        }
    }
    println!("sum of priorities: {}", sum);
}

fn main() {
    part_one();
    part_two();
}
