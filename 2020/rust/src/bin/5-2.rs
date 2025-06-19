use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/5.txt").expect("Couldn't read the file");
    let mut seats = [[false; 8]; 128];
    for l in input.lines() {
        let mut row_upper = 127;
        let mut row_lower = 0;
        for c in l.chars() {
            if c == 'F' {
                row_upper -= (row_upper - row_lower) / 2 + 1;
            } else if c == 'B' {
                row_lower += (row_upper - row_lower) / 2 + 1;
            }
        }

        let mut column_upper = 7;
        let mut column_lower = 0;
        for c in l.chars() {
            if c == 'L' {
                column_upper -= (column_upper - column_lower) / 2 + 1;
            } else if c == 'R' {
                column_lower += (column_upper - column_lower) / 2 + 1;
            };
        }
        seats[row_lower][column_lower] = true;
    }
    for row in 10..118 {
        for column in 0..8 {
            if !seats[row][column] {
                println!("{}", row * 8 + column);
            }
        }
    }
}
