use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/3.txt").expect("Couldn't read the file");
    let cleand_input = input.replace("\n", ""); // compiler told me to do this (and I found out why)
    let bytes = cleand_input.as_bytes();
    let mut total: usize = 1;
    for (right, down) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut count: usize = 0;
        while y < input.lines().count() {
            if bytes[x % 31 + y * 31] == b'#' {
                count += 1;
            }
            x += right;
            y += down;
        }
        total *= count;
    }
    println!("{}", total)
}
