use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/2.txt").expect("Couldn't read the file");

    let re_letter = Regex::new(r" (?<letter>\w+):").unwrap();
    let re_min = Regex::new(r"(?<number>\w+)-").unwrap();
    let re_max = Regex::new(r"-(?<number>\w+)").unwrap();
    println!(
        "{:?}",
        input
            .lines()
            .filter(|i| {
                i.chars()
                    .filter(|c| {
                        *c == re_letter.captures(i).unwrap()["letter"]
                            .chars()
                            .next()
                            .unwrap()
                    })
                    .count()
                    >= re_min.captures(i).unwrap()["number"]
                        .parse::<usize>()
                        .unwrap()
                    && i.chars()
                        .filter(|c| {
                            *c == re_letter.captures(i).unwrap()["letter"]
                                .chars()
                                .next()
                                .unwrap()
                        })
                        .count()
                        <= re_max.captures(i).unwrap()["number"]
                            .parse::<usize>()
                            .unwrap()
            })
            .count()
    );
}
