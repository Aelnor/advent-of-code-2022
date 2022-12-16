use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug)]
struct Cave {
    flow_rate: u32,
    connections: Vec<String>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct State {
    minutes: u32,
    location: String,
    rate: u32,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct State2 {
    minutes: u32,
    my_position: String,
    elefant_position: String,
    rate: u32,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct State3 {
    minutes: u32,
    my_position: String,
    elefant_position: String,
    opened: u32,
    score: u32,
    flow_rate: u32,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct State4 {
    my_position: String,
    elefant_position: String,
    minutes: u32,
}

fn traverse(
    map: &HashMap<String, Cave>,
    current_room: &str,
    minutes: u32,
    score: u32,
    flow_rate: u32,
    opened_valves: &HashSet<String>,
    cache: &mut HashMap<State, u32>,
) -> u32 {
    if minutes == 0 {
        return score;
    }

    let state = State {
        minutes,
        location: String::from(current_room),
        rate: flow_rate,
    };

    if let Some(points) = cache.get(&state) {
        if score <= *points {
            return *points;
        }
    }

    cache.insert(state, score);
    let room = map.get(current_room).unwrap();
    let mut cur_max = 0;
    let new_score = score + flow_rate;

    if room.flow_rate != 0 && !opened_valves.contains(current_room) {
        let new_flow_rate = flow_rate + room.flow_rate;
        let mut new_opened_valves = opened_valves.clone();
        new_opened_valves.insert(String::from(current_room));
        cur_max = traverse(
            map,
            current_room,
            minutes - 1,
            new_score,
            new_flow_rate,
            &new_opened_valves,
            cache,
        );
    }

    for conn in &room.connections {
        cur_max = std::cmp::max(
            cur_max,
            traverse(
                map,
                conn,
                minutes - 1,
                new_score,
                flow_rate,
                opened_valves,
                cache,
            ),
        );
    }

    cur_max
}

fn traverse2(
    map: &HashMap<String, Cave>,
    bitmask: &HashMap<String, u32>,
    max_flow_rate: u32,
) -> u32 {
    let mut states = Vec::new();
    states.push(State3 {
        minutes: 26,
        my_position: String::from("AA"),
        elefant_position: String::from("AA"),
        opened: 0,
        score: 0,
        flow_rate: 0,
    });

    let mut cache: HashMap<State4, u32> = HashMap::new();

    let mut max_pressure = 0;

    while !states.is_empty() {
        let state = states.pop().unwrap();

        let cache_state = State4 {
            my_position: state.my_position.clone(),
            elefant_position: state.elefant_position.clone(),
            minutes: state.minutes,
        };
        if let Some(points) = cache.get(&cache_state) {
            if state.score <= *points {
                continue;
            }
        }

        *cache.entry(cache_state).or_insert(0) = state.score;

        if state.minutes == 1 {
            max_pressure = std::cmp::max(max_pressure, state.score);
            continue;
        }

        if state.flow_rate == max_flow_rate {
            let score = state.score + (state.minutes - 1) * state.flow_rate;
            max_pressure = std::cmp::max(max_pressure, score);
            continue;
        }

        let my_room = map.get(&state.my_position).unwrap();
        let elefant_room = map.get(&state.elefant_position).unwrap();

        if (my_room.flow_rate != 0)
            && (state.opened & *bitmask.get(&state.my_position).unwrap() == 0)
        {
            let mut new_opened_valves = state.opened | *bitmask.get(&state.my_position).unwrap();

            if (elefant_room.flow_rate != 0)
                && (new_opened_valves & *bitmask.get(&state.elefant_position).unwrap() == 0)
            {
                new_opened_valves |= *bitmask.get(&state.elefant_position).unwrap();
                let new_score =
                    state.score + state.flow_rate + elefant_room.flow_rate + my_room.flow_rate;
                states.push(State3 {
                    minutes: state.minutes - 1,
                    my_position: state.my_position.clone(),
                    elefant_position: state.elefant_position.clone(),
                    opened: new_opened_valves,
                    score: new_score,
                    flow_rate: state.flow_rate + elefant_room.flow_rate + my_room.flow_rate,
                });
                new_opened_valves &= !*bitmask.get(&state.elefant_position).unwrap();
            }

            for con in &elefant_room.connections {
                states.push(State3 {
                    minutes: state.minutes - 1,
                    my_position: state.my_position.clone(),
                    elefant_position: con.clone(),
                    opened: new_opened_valves,
                    score: state.score + state.flow_rate + my_room.flow_rate,
                    flow_rate: state.flow_rate + my_room.flow_rate,
                })
            }
        }

        for con in &my_room.connections {
            if elefant_room.flow_rate != 0
                && (state.opened & *bitmask.get(&state.elefant_position).unwrap() == 0)
            {
                let new_opened_valves =
                    state.opened | *bitmask.get(&state.elefant_position).unwrap();
                states.push(State3 {
                    minutes: state.minutes - 1,
                    my_position: con.clone(),
                    elefant_position: state.elefant_position.clone(),
                    opened: new_opened_valves,
                    score: state.score + state.flow_rate + elefant_room.flow_rate,
                    flow_rate: state.flow_rate + elefant_room.flow_rate,
                });
            }

            for elefant_con in &elefant_room.connections {
                states.push(State3 {
                    minutes: state.minutes - 1,
                    my_position: con.clone(),
                    elefant_position: elefant_con.clone(),
                    opened: state.opened,
                    score: state.score + state.flow_rate,
                    flow_rate: state.flow_rate,
                })
            }
        }
    }

    max_pressure
}

fn main() {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut map = HashMap::new();
    let mut max_flow_rate = 0;
    let mut bitmask = HashMap::new();
    let mut bitmask_index = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let split = line.split(" ").collect::<Vec<&str>>();

        let name = String::from(split[1]);
        let mut cave: Cave = Default::default();

        let flow_rate = split[4].split("=").nth(1).unwrap();

        cave.flow_rate = flow_rate[0..flow_rate.len() - 1].parse::<u32>().unwrap();

        for i in 9..split.len() {
            if split[i].chars().last().unwrap() == ',' {
                let conn = String::from(&split[i][0..split[i].len() - 1]);
                cave.connections.push(conn);
                continue;
            }
            cave.connections.push(String::from(split[i]));
        }
        max_flow_rate += cave.flow_rate;
        if cave.flow_rate != 0 {
            bitmask.insert(name.clone(), 1 << bitmask_index);
            bitmask_index += 1;
        }

        map.insert(name, cave);
    }

    let mut cache: HashMap<State, u32> = HashMap::new();
    let result = traverse(&map, "AA", 30, 0, 0, &HashSet::new(), &mut cache);
    println!("most pressure release: {}", result);
    println!(
        "most pressure release: {}",
        traverse2(&map, &bitmask, max_flow_rate)
    );
}
