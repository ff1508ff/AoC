use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("../inputs/1.txt").expect("Couldn't read the file");

    let mut memory: HashSet<u32> = HashSet::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let number: u32 = line.parse::<u32>().unwrap();
        let div = 2020 - number;
        if memory.contains(&div) {
            println!("{}", number * div);
            return;
        }

        memory.insert(number);
    }
    println!("nope");
}
