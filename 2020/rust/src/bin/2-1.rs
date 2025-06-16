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
                    let min = caps["min"].parse::<usize>().unwrap();
                    let max = caps["max"].parse::<usize>().unwrap();
                    let letter = caps["letter"].chars().next().unwrap();
                    let password = &caps["password"];
                    let count = password.matches(letter).count();
                    count >= min && count <= max
                } else {
                    false
                }
            })
            .count()
    )
}
