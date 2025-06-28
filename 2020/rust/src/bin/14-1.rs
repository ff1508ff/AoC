use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("../inputs/14.txt").expect("Couldn't read the file");
    let re_mem = Regex::new(r"mem\[(?<address>\w+)] = (?<value>\w+)").unwrap();
    let re_mask = Regex::new(r"mask = (?<mask>\w+)").unwrap();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = String::new();

    for line in input.lines() {
        if let Some(values) = re_mask.captures(line) {
            mask = values["mask"].to_owned();
        } else {
            let address: u64 = re_mem.captures(line).unwrap()["address"].parse().unwrap();
            let mut value: u64 = re_mem.captures(line).unwrap()["value"].parse().unwrap();
            for (i, c) in mask.chars().rev().enumerate() {
                match c {
                    '1' => value |= 1 << i,
                    '0' => value &= !(1 << i),
                    'X' => (),
                    _ => panic!("Unexpected value: {}", c),
                }
            }
            memory.insert(address, value);
        }
    }

    println!("{}", memory.values().sum::<u64>());
}
