use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("../inputs/8.txt").expect("Couldn't read the file");

    let script: Vec<(&str, &str)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let mut seen = HashSet::new();
    let mut accumulator: i32 = 0;
    recursive(0, &script, &mut accumulator, &mut seen);

    println!("{}", accumulator);
}

fn recursive(
    line: usize,
    script: &Vec<(&str, &str)>,
    accumulator: &mut i32,
    seen: &mut HashSet<usize>,
) {
    if seen.contains(&line) {
        return;
    }
    seen.insert(line);

    if line >= script.len() {
        return;
    }

    let (command, number) = script[line];
    let value = number.parse::<i32>().unwrap();

    match command {
        "acc" => {
            *accumulator += value;
            recursive(line + 1, script, accumulator, seen);
        }
        "jmp" => {
            let target = if value.is_negative() {
                line as i32 + value
            } else {
                line as i32 + value
            };
            recursive(target as usize, script, accumulator, seen);
        }
        "nop" => {
            recursive(line + 1, script, accumulator, seen);
        }
        _ => panic!("Unknown instruction"),
    }
}
