use std::{collections::HashSet, fs, usize};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("../inputs/16.txt").expect("Couldn't read the file");

    let re_rules =
        Regex::new(r"(?<number1>\w+)-(?<number2>\w+) or (?<number3>\w+)-(?<number4>\w+)").unwrap();
    let mut valid: HashSet<usize> = HashSet::new();

    for line in input.split("\n\n").nth(0).unwrap().lines() {
        let rules = re_rules.captures(line).unwrap();
        valid.extend(
            rules["number1"].parse::<usize>().unwrap()..=rules["number2"].parse::<usize>().unwrap(),
        );
        valid.extend(
            rules["number3"].parse::<usize>().unwrap()..=rules["number4"].parse::<usize>().unwrap(),
        );
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
