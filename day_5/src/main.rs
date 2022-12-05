use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut main_vec = Vec::new();
    let mut instructions = Vec::new();
    let mut instructions_mode = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if instructions_mode {
            instructions.push(line);
            continue;
        }

        if line.is_empty() {
            instructions_mode = true;
            continue;
        }

        if line.chars().nth(1).unwrap() == '1' {
            continue;
        }

        let mut i = 0;

        loop {
            let line_index = i * 4 + 1;
            if line.len() <= line_index {
                break;
            }

            let c = line.chars().nth(line_index).unwrap();

            if main_vec.len() <= i {
                main_vec.push(vec![]);
            }

            if c != ' ' {
                main_vec[i].push(c);
            }

            i += 1;
        }
    }
    for i in 0..main_vec.len() {
        main_vec[i].reverse();
    }

    for line in instructions {
        let mut split = line.split(" ");
        let count = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        let mut temp_vec = Vec::new();
        for _ in 0..count {
            temp_vec.push(main_vec[from].pop().unwrap());
        }
        temp_vec.reverse();
        main_vec[to].append(&mut temp_vec);
    }

    for mut v in main_vec {
        print!("{}", v.pop().unwrap());
    }
    println!("");
}
