use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone)]
struct Scanner {
    position: Point,
    radius: i64,
}

#[derive(Debug)]
struct Range {
    start_point: i64,
    end_point: i64,
}

impl Point {
    fn distance_to(&self, p: &Point) -> i64 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }
}

fn merge_ranges(ranges: &mut Vec<Range>) {
    ranges.sort_by(|a, b| a.start_point.cmp(&b.start_point));
    let mut index = 0;
    while index < ranges.len() - 1 {
        if ranges[index + 1].start_point <= ranges[index].end_point {
            ranges[index].start_point =
                std::cmp::min(ranges[index].start_point, ranges[index + 1].start_point);
            ranges[index].end_point =
                std::cmp::max(ranges[index].end_point, ranges[index + 1].end_point);

            ranges.remove(index + 1);
            continue;
        }
        index += 1;
    }
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut ranges = Vec::new();
    let y = 2000000;
    let mut beacons_on_this_row = std::collections::HashSet::new();

    let mut scanners = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let split: Vec<&str> = line.split(' ').collect();

        let scanner = Point {
            x: split[2]
                .chars()
                .skip(2)
                .take(split[2].len() - 3)
                .collect::<String>()
                .parse::<i64>()
                .unwrap(),
            y: split[3]
                .chars()
                .skip(2)
                .take(split[3].len() - 3)
                .collect::<String>()
                .parse::<i64>()
                .unwrap(),
        };

        let beacon = Point {
            x: split[8]
                .chars()
                .skip(2)
                .take(split[8].len() - 3)
                .collect::<String>()
                .parse::<i64>()
                .unwrap(),
            y: split[9]
                .chars()
                .skip(2)
                .take(split[9].len() - 2)
                .collect::<String>()
                .parse::<i64>()
                .unwrap(),
        };

        if beacon.y == y {
            beacons_on_this_row.insert(beacon.x);
        }

        let radius = beacon.distance_to(&scanner);
        let distance = scanner.distance_to(&Point { x: scanner.x, y });

        scanners.push(Scanner {
            position: scanner.clone(),
            radius,
        });

        if radius < distance {
            continue;
        }

        ranges.push(Range {
            start_point: scanner.x - (radius - distance),
            end_point: scanner.x + (radius - distance),
        });
    }

    merge_ranges(&mut ranges);
    let mut result = 0;
    for range in ranges {
        result += range.end_point - range.start_point + 1;
    }
    println!(
        "possible positions: {}",
        result - beacons_on_this_row.len() as i64
    );

    let max = 4000000;
    for row in 0..=max {
        let mut ranges = Vec::new();
        for scanner in &scanners {
            let distance = scanner.position.distance_to(&Point {
                x: scanner.position.x,
                y: row,
            });
            if scanner.radius < distance {
                continue;
            }
            let mut range = Range {
                start_point: scanner.position.x - (scanner.radius - distance),
                end_point: scanner.position.x + (scanner.radius - distance),
            };
            if range.start_point > max || range.end_point < 0 {
                continue;
            }
            range.start_point = std::cmp::max(range.start_point, 0);
            range.end_point = std::cmp::min(range.end_point, max);
            ranges.push(range);
        }
        merge_ranges(&mut ranges);
        if ranges.len() != 1 {
            println!("frequency {}", (ranges[0].end_point + 1) * 4000000 + row);
            return;
        }
    }
}
