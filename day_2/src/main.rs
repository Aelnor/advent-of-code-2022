use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

struct Move {
    opponent: Shape,
    my: Shape,
}

impl Shape {
    fn shape_to_lose(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn shape_to_win(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn from(letter: &str) -> Self {
        match letter {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Move {
    fn from(mv: &Vec<&str>) -> Self {
        let mut result = Move {
            opponent: Shape::from(mv[0]),
            my: Shape::Rock,
        };

        match mv[1] {
            "X" => result.my = Shape::Rock,
            "Y" => result.my = Shape::Paper,
            "Z" => result.my = Shape::Scissors,
            _ => unreachable!(),
        }

        return result;
    }

    fn from_v2(mv: &Vec<&str>) -> Self {
        let mut result = Move {
            opponent: Shape::from(mv[0]),
            my: Shape::Rock,
        };
        match mv[1] {
            "X" => result.my = result.opponent.shape_to_lose(),
            "Y" => result.my = result.opponent,
            "Z" => result.my = result.opponent.shape_to_win(),
            _ => unreachable!(),
        }

        return result;
    }

    fn calc_score(&self) -> u32 {
        let result = match self.my {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        if self.my == self.opponent {
            return result + 3;
        }

        if self.i_won() {
            return result + 6;
        }

        result
    }

    fn i_won(&self) -> bool {
        (self.my == Shape::Rock && self.opponent == Shape::Scissors)
            || (self.my == Shape::Paper && self.opponent == Shape::Rock)
            || (self.my == Shape::Scissors && self.opponent == Shape::Paper)
    }
}

fn main() {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    let mut points = 0;
    let mut points2 = 0;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let split: Vec<&str> = line.split(" ").collect();

        let mv = Move::from(&split);
        let mv2 = Move::from_v2(&split);

        points += mv.calc_score();
        points2 += mv2.calc_score();
    }

    println!("Points: {}", points);
    println!("Points-2: {}", points2);
}
