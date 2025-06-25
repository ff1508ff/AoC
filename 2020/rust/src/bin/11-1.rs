use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/11.txt").expect("Couldn't read the file");

    #[derive(Clone, PartialEq)]
    enum Status {
        Floor,
        Empty,
        Occupied,
    }

    use Status::*;

    let mut board: Vec<Vec<Status>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => Occupied, // skip 1 iteration
                    '.' => Floor,
                    _ => panic!("Unknown character {}", c),
                })
                .collect()
        })
        .collect();

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut changed = true;
    while changed {
        changed = false;
        let prev = board.clone();

        for y in 0..board.len() {
            for x in 0..board[0].len() {
                if prev[y][x] == Floor {
                    continue;
                }

                let mut neighbor = 0;
                for (my, mx) in &directions {
                    let ny = y as isize + my;
                    let nx = x as isize + mx;
                    if ny >= 0
                        && ny < board.len() as isize
                        && nx >= 0
                        && nx < board[0].len() as isize
                        && prev[ny as usize][nx as usize] == Occupied
                    {
                        neighbor += 1;
                    }
                }

                board[y][x] = match prev[y][x] {
                    Empty if neighbor == 0 => {
                        changed = true;
                        Occupied
                    }
                    Occupied if neighbor >= 4 => {
                        changed = true;
                        Empty
                    }
                    _ => prev[y][x].clone(),
                };
            }
        }
    }

    let total = board.iter().flatten().filter(|c| **c == Occupied).count();

    println!("{}", total);
}
