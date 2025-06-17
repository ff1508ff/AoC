use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/3.txt").expect("Couldn't read the file");
    let mut x: usize = 0;
    let mut count: usize = 0;
    for line in input.lines() {
        if x >= 31 {
            x -= 31;
        }
        if line.chars().nth(x).unwrap() == '#' {
            count += 1;
        }
        x += 3;
    }
    println!("{}", count)
}
