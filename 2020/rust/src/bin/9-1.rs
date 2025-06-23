use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/9.txt").expect("Couldn't read the file");
    let mut list: Vec<usize> = vec![];
    input.lines().for_each(|l| list.push(l.parse().unwrap()));

    let mut down: usize = 0;
    let mut up: usize = 25;
    let mut found: bool = false;

    while true {
        'outer: for a in down..up {
            for b in a..up {
                if list[up] == list[a] + list[b] {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            println!("{}", list[up]);
            return;
        }
        found = false;
        down += 1;
        up += 1;
    }
}
