use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/13.txt").expect("Couldn't read the file");
    let your_time = input.lines().next().unwrap().parse::<usize>().unwrap();
    let mut nearest_time = usize::MAX;
    let mut nearest_bus = 0;
    for bus in input.lines().last().unwrap().split(',') {
        if bus == "x" {
            continue;
        }

        let bus = bus.parse::<usize>().unwrap();
        // https://users.rust-lang.org/t/solved-rust-round-usize-to-nearest-multiple-of-8/25549 +
        // clippy
        if your_time.div_ceil(bus) * bus < nearest_time {
            nearest_time = your_time.div_ceil(bus) * bus;
            nearest_bus = bus;
        }
    }
    println!("{}", nearest_bus * (nearest_time - your_time));
}
