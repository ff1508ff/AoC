use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("../inputs/1.txt").expect("Couldn't read the file");
    let mut memory: HashMap<u32, u32> = HashMap::new();
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let line: u32 = line.parse::<u32>().unwrap();
        let div = 2020 - line;
        if memory.contains_key(&line) {
            println!("{}", line * memory.get(&line).unwrap());
            return;
        } else {
            memory.insert(div, line);
        }
    }
    println!("nope");
}
