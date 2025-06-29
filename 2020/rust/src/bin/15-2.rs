use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/15.txt").expect("Couldn't read the file");
    let input: Vec<usize> = input
        .replace("\n", "")
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut memory = vec![0usize; 30000000];

    let mut time: usize = 1;
    for &number in &input[..input.len() - 1] {
        memory[number] = time;
        time += 1;
    }

    let mut last_number = *input.last().unwrap();

    while time != 30000000 {
        let next = if memory[last_number] != 0 {
            time - memory[last_number]
        } else {
            0
        };
        memory[last_number] = time;
        last_number = next;
        time += 1;
    }
    println!("{}", last_number)
}
