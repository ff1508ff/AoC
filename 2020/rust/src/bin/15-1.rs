use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("../inputs/15.txt").expect("Couldn't read the file");
    let input: Vec<usize> = input
        .replace("\n", "")
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut memory: HashMap<usize, usize> = HashMap::new();

    let mut time: usize = 1;
    let mut last_number: usize = 0;
    for &number in &input[..input.len() - 1] {
        memory.insert(number, time);
        time += 1;
    }

    let mut last_number = *input.last().unwrap();

    while time != 2020 {
        let next = if let Some(last_time) = memory.get(&last_number) {
            time - *last_time
        } else {
            0
        };
        memory.insert(last_number, time);
        last_number = next;
        time += 1;
    }
    println!("{}", last_number)
}
