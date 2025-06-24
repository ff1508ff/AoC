use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/10.txt").expect("Couldn't read the file");
    let mut input: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    // I like this
    input.push(0); // outlet
    input.sort_unstable();
    input.push(input.last().unwrap() + 3); // laptop

    let mut jolt1 = 0;
    let mut jolt3 = 0;

    for pair in input.windows(2) {
        match pair[1] - pair[0] {
            1 => jolt1 += 1,
            3 => jolt3 += 1,
            _ => {}
        }
    }
    println!("{}", jolt1 * jolt3);
}
