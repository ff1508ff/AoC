use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("../inputs/6.txt").expect("Couldn't read the file");
    let mut total = 0;
    for group in input.split("\n\n") {
        let group_size = group.lines().count();
        let group = group.replace("\n", "");
        let mut memory: HashMap<char, usize> = HashMap::new();
        for c in group.chars() {
            *memory.entry(c).or_default() += 1;
        }
        total += memory.iter().filter(|c| *c.1 == group_size).count();
    }
    println!("{}", total)
}
