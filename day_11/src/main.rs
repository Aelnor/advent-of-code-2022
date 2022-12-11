// only solution to the second part of the puzzle.
// I was lazy enough not to keep the first one separately.
// It's trivial though: just reimplement `Number` for u64 and change rounds number and add / 3 for
// the new val
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

#[derive(Default, Debug)]
struct Monkey {
    items: Vec<Number>,
    operation: char,
    operation_argument: String,
    divisible_by: u32,
    throw: [usize; 2],
    inspect_count: u64,
}

#[derive(Clone, Debug)]
struct Number {
    remainders: HashMap<u32, u32>,
}

const DIVISORS: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

impl Number {
    fn from(value: u32) -> Self {
        let mut result = Number {
            remainders: HashMap::new(),
        };

        for d in &DIVISORS {
            result.remainders.insert(*d, value % d);
        }
        result
    }

    fn add(&mut self, value: u32) {
        for d in &DIVISORS {
            self.remainders
                .entry(*d)
                .and_modify(|f| *f = (*f + value) % d);
        }
    }

    fn multiply_by(&mut self, value: u32) {
        for d in &DIVISORS {
            self.remainders
                .entry(*d)
                .and_modify(|f| *f = (*f * (value % d)) % d);
        }
    }
    fn square(&mut self) {
        for d in &DIVISORS {
            self.remainders.entry(*d).and_modify(|f| *f = (*f * *f) % d);
        }
    }

    fn remainder(&self, value: u32) -> u32 {
        *self.remainders.get(&value).unwrap()
    }
}

fn perform_operation(operation: char, argument: &str, value: &Number) -> Number {
    let mut result = value.clone();
    match operation {
        '+' => result.add(argument.parse().unwrap()),
        '*' => match argument {
            "old" => result.square(),
            _ => result.multiply_by(argument.parse().unwrap()),
        },
        _ => unreachable!(),
    }
    result
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut monkeys = Vec::new();
    let mut monkey: Monkey = Default::default();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            monkeys.push(monkey);
            monkey = Default::default();
            continue;
        }

        let split: Vec<&str> = line.trim().split(" ").collect();
        match split[0] {
            "Monkey" => continue,
            "Starting" => {
                for i in 2..split.len() {
                    if split[i].ends_with(",") {
                        monkey.items.push(Number::from(
                            split[i][0..split[i].len() - 1].parse::<u32>().unwrap(),
                        ));
                    } else {
                        monkey
                            .items
                            .push(Number::from(split[i].parse::<u32>().unwrap()));
                    }
                }
            }
            "Operation:" => {
                monkey.operation = split[4].chars().nth(0).unwrap();
                monkey.operation_argument = String::from(split[5]);
            }
            "Test:" => {
                assert_eq!(split[1], "divisible");
                monkey.divisible_by = split[3].parse().unwrap();
            }
            "If" => {
                let num = split[5].parse().unwrap();
                if split[1] == "true:" {
                    monkey.throw[0] = num;
                } else {
                    monkey.throw[1] = num;
                }
            }
            _ => unreachable!("found {}", split[0]),
        }
    }

    let round_number = 10000;
    for _ in 0..round_number {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let new_val = perform_operation(
                    monkeys[i].operation,
                    &monkeys[i].operation_argument,
                    &monkeys[i].items[j],
                );
                monkeys[i].inspect_count += 1;

                let new_monkey = if new_val.remainder(monkeys[i].divisible_by) == 0 {
                    monkeys[i].throw[0]
                } else {
                    monkeys[i].throw[1]
                };

                assert_ne!(i, new_monkey);

                monkeys[new_monkey].items.push(new_val);
            }
            monkeys[i].items.clear();
        }
    }
    monkeys.sort_by(|l, r| r.inspect_count.cmp(&l.inspect_count));
    println!(
        "monkey business: {}",
        monkeys[0].inspect_count * monkeys[1].inspect_count
    );
}
