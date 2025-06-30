use std::{collections::HashSet, fs};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("../inputs/16.txt").expect("Couldn't read the file");

    let re_rules =
        Regex::new(r"(?<number1>\w+)-(?<number2>\w+) or (?<number3>\w+)-(?<number4>\w+)").unwrap();
    let mut valid: HashSet<usize> = HashSet::new();

    for line in input.split("\n\n").nth(0).unwrap().lines() {
        let rules = re_rules.captures(line).unwrap();
        (rules["number1"].parse().unwrap()..=rules["number2"].parse().unwrap())
            .for_each(|n| _ = valid.insert(n));
        (rules["number3"].parse().unwrap()..=rules["number4"].parse().unwrap())
            .for_each(|n| _ = valid.insert(n));
    }

    let mut result = 0;

    for line in input.split("\n\n").nth(2).unwrap().lines().skip(1) {
        for number in line.split(",") {
            let number = number.parse::<usize>().unwrap();
            if !valid.contains(&number) {
                result += number;
            }
        }
    }
    println!("{}", result);
}
