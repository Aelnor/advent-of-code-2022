use std::fs::File;
use std::io::{BufRead, BufReader};

fn compare_and_increase_signal_strength(cycle: i32, x: i32, result: &mut i32) {
    if cycle == 20 || (cycle - 20) % 40 == 0 {
        *result += cycle * x;
    }
}

fn draw_pixel(cycle: i32, x: i32) {
    let pos = cycle % 40;
    if pos >= x - 1 && pos <= x + 1 {
        print!("#");
    } else {
        print!(" ");
    }
    if pos == 39 {
        println!("");
    }
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut x = 1;
    let mut cycle = 0;
    let mut result = 0;

    for line in reader.lines() {
        draw_pixel(cycle, x);
        cycle += 1;
        let line = line.unwrap();
        let split: Vec<&str> = line.split(" ").collect();

        if split[0] == "addx" {
            compare_and_increase_signal_strength(cycle, x, &mut result);
            draw_pixel(cycle, x);
            cycle += 1;
        }

        compare_and_increase_signal_strength(cycle, x, &mut result);
        if split[0] == "addx" {
            x += split[1].parse::<i32>().unwrap();
        }
    }

    println!("signal strength {}", result);
}
