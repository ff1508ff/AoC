use regex::Regex;
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("../inputs/6.txt").expect("Couldn't read the file");
    let mut total = 0;
    for group in input.split("\n\n") {
        let group = group.replace("\n", "");
        let mut memory: HashSet<char> = HashSet::new();
        for c in group.chars() {
            memory.insert(c);
        }
        total += memory.iter().count();
    }
    println!("{}", total)
}
