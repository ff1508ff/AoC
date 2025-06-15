use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/1.txt").expect("Couldn't read the file");

    let numbers: Vec<u32> = input
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .collect();

    for a in 0..numbers.len() {
        for b in a..numbers.len() {
            for c in b..numbers.len() {
                if numbers[a] + numbers[b] + numbers[c] == 2020 {
                    println!("{}", numbers[a] * numbers[b] * numbers[c]);
                    return;
                }
            }
        }
    }

    println!("nope");
}
