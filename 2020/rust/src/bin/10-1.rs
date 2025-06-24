use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/10.txt").expect("Couldn't read the file");
    let mut input: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();
    input.sort_unstable();
    let mut jolt1 = 0;
    let mut jolt3 = 0;
    let mut old = 0;
    for number in input {
        if 1 == number - old {
            jolt1 += 1;
        } else if 3 == number - old {
            jolt3 += 1;
        }
        old = number;
    }
    jolt3 += 1;
    println!("{}", jolt1 * jolt3);
}
