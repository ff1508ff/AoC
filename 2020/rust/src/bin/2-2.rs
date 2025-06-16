use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/2.txt").expect("Couldn't read the file");

    let re = Regex::new(r"(?<min>\d+)-(?<max>\d+)\s+(?<letter>\w):\s+(?<password>\w+)").unwrap();

    println!(
        "{}",
        input
            .lines()
            .filter(|line| {
                if let Some(caps) = re.captures(line) {
                    let pos1 = caps["min"].parse::<usize>().unwrap() - 1;
                    let pos2 = caps["max"].parse::<usize>().unwrap() - 1;
                    let letter = caps["letter"].chars().next().unwrap();
                    let password = &caps["password"];
                    let first = password.chars().nth(pos1).unwrap_or('0') == letter;
                    let second = password.chars().nth(pos2).unwrap_or('0') == letter;
                    // I think this is so smart
                    first ^ second
                } else {
                    false
                }
            })
            .count()
    );
}
