use std::collections::HashSet;

fn main() {
    let line: Vec<char> = std::fs::read_to_string("data").unwrap().chars().collect();
    let message_len = 14;

    for i in message_len - 1..line.len() {
        let mut set = HashSet::new();
        for j in i - (message_len - 1)..i + 1 {
            set.insert(line[j]);
        }
        if set.len() == message_len {
            println!("{}", i + 1);
            break;
        }
    }
}
