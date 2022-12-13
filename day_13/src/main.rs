use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Tokenizer<'a> {
    data: &'a str,
    index: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(s: &'a str) -> Self {
        return Tokenizer { data: s, index: 0 };
    }

    fn next_token(&mut self) -> Option<String> {
        if self.index == self.data.len() {
            return None;
        }
        let mut result = String::new();
        let mut level = 0;

        loop {
            if self.index == self.data.len() {
                if result.is_empty() {
                    return None;
                }

                return Some(result);
            }

            let current_char = self.data.chars().nth(self.index)?;

            match current_char {
                ',' => {
                    if level == 0 {
                        self.index += 1;
                        return Some(result);
                    }
                }
                '[' => {
                    level += 1;
                }
                ']' => {
                    level -= 1;
                }
                _ => {}
            }
            result.push(current_char);
            self.index += 1;
        }
    }
}

fn is_list(s: &str) -> bool {
    s.starts_with('[') && s.ends_with(']')
}

fn get_value(s: &str) -> String {
    if is_list(s) {
        return String::from(&s[1..s.len() - 1]);
    }
    String::from(s)
}

fn compare(lhs: &str, rhs: &str) -> Ordering {
    let mut left = Tokenizer::new(lhs);
    let mut right = Tokenizer::new(rhs);
    loop {
        let left_token = left.next_token();
        let right_token = right.next_token();

        if left_token.is_none() && right_token.is_none() {
            return Ordering::Equal;
        }
        if left_token.is_none() && right_token.is_some() {
            return Ordering::Less;
        }

        if left_token.is_some() && right_token.is_none() {
            return Ordering::Greater;
        }

        let left_token = left_token.unwrap();
        let right_token = right_token.unwrap();

        if is_list(&left_token) || is_list(&right_token) {
            let res = compare(&get_value(&left_token), &get_value(&right_token));
            if res != Ordering::Equal {
                return res;
            }
            continue;
        }

        // should be both numbers
        let left_number = left_token.parse::<u32>().unwrap();
        let right_number = right_token.parse::<u32>().unwrap();
        if left_number < right_number {
            return Ordering::Less;
        }
        if left_number > right_number {
            return Ordering::Greater;
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut pair = Vec::new();

    let mut result = 0;
    let mut pair_number = 1;

    let mut part_two_vector = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            part_two_vector.push(String::from(&line));
            pair.push(String::from(line));
            continue;
        }

        if compare(&pair[0], &pair[1]) == Ordering::Less {
            result += pair_number;
        }

        pair_number += 1;
        pair.clear();
    }

    println!("part one: {}", result);
    part_two_vector.push(String::from("[[2]]"));
    part_two_vector.push(String::from("[[6]]"));

    part_two_vector.sort_by(|lhs, rhs| compare(lhs, rhs));

    result = 0;

    for (index, line) in part_two_vector.iter().enumerate() {
        if line == "[[2]]" {
            result = index + 1;
            continue;
        }

        if line == "[[6]]" {
            println!("part two: {}", result * (index + 1));
            return;
        }
    }
}
