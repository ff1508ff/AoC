use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("../inputs/7.txt").expect("Couldn't read the file");
    let re = Regex::new(r"(?<color>\w+ \w+) bag").unwrap();

    let mut memory: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let bags: Vec<_> = re.captures_iter(line).collect();
        if bags.is_empty() {
            continue;
        }

        let parent = &bags[0]["color"];

        for bag in &bags[1..] {
            let inner = &bag["color"];
            memory
                .entry(inner.to_string())
                .or_default()
                .push(parent.to_string());
        }
    }

    let mut seen = HashSet::new();
    recursive("shiny gold", &memory, &mut seen);

    println!("{}", seen.len());
}

fn recursive(color: &str, memory: &HashMap<String, Vec<String>>, seen: &mut HashSet<String>) {
    if let Some(parents) = memory.get(color) {
        for parent in parents {
            if seen.insert(parent.clone()) {
                recursive(parent, memory, seen);
            }
        }
    }
}
