use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/9.txt").expect("Couldn't read the file");
    let mut list: Vec<usize> = vec![];
    input.lines().for_each(|l| list.push(l.parse().unwrap()));

    let mut down: usize = 0;
    let mut found: bool = false;
    let mut solution: usize = 0;

    for down in 0..list.len() {
        'outer: for a in down..down + 25 {
            for b in a..down + 25 {
                if list[down + 25] == list[a] + list[b] {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            solution = list[down + 25];
            break;
        }
        found = false;
    }

    for down in 0..list.len() {
        let mut sum = list[down];
        for up in down + 1..list.len() {
            sum += list[up];
            if sum == solution {
                let range = &list[down..=up];
                println!(
                    "{}",
                    range.iter().max().unwrap() + range.iter().min().unwrap()
                );
                return;
            }
        }
    }
}
