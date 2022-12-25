use std::fs::File;
use std::io::{BufRead, BufReader};

fn from_snafu(s: &str) -> isize {
    let mut pow = 0;
    let mut result = 0;

    for c in s.chars().rev() {
        result += match c {
            '=' => -2 * 5_isize.pow(pow),
            '-' => -1 * 5_isize.pow(pow),
            _ => c.to_digit(5).unwrap() as isize * 5_isize.pow(pow),
        };
        pow += 1;
    }
    result
}
fn to_snafu(number: isize) -> String {
    let mut result = String::new();
    let mut num = number;
    while num != 0 {
        let v = (num + 2) % 5 - 2;
        let digit = match v {
            -2 => '=',
            -1 => '-',
            _ => char::from_digit(v as u32, 5).unwrap(),
        };
        result.insert(0, digit);
        num = (num + 2) / 5;
    }
    result
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = 0;
    for line in reader.lines() {
        result += from_snafu(&line.unwrap());
    }

    println!("result: {}", to_snafu(result));
}
