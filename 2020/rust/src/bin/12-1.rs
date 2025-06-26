use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/12.txt").expect("Couldn't read the file");

    #[derive(PartialEq, Clone)]
    enum Direction {
        North,
        East,
        South,
        West,
    }
    use Direction::*;

    impl Direction {
        fn turn_right(&mut self, times: usize) {
            let directions = [North, East, South, West];
            let idx = directions.iter().position(|d| d == self).unwrap();
            *self = directions[(idx + times) % 4].clone()
        }

        fn turn_left(&mut self, times: usize) {
            self.turn_right(4 - (times % 4));
        }
    }

    struct Ferry {
        x: i32,
        y: i32,
        direction: Direction,
    }

    let mut ferry = Ferry {
        x: 0,
        y: 0,
        direction: East,
    };

    input.lines().for_each(|i| match i.chars().next().unwrap() {
        'N' => ferry.y += i[1..].parse::<i32>().unwrap(),
        'S' => ferry.y -= i[1..].parse::<i32>().unwrap(),
        'E' => ferry.x += i[1..].parse::<i32>().unwrap(),
        'W' => ferry.x -= i[1..].parse::<i32>().unwrap(),
        'L' => ferry
            .direction
            .turn_left(i[1..].parse::<usize>().unwrap() / 90),
        'R' => ferry
            .direction
            .turn_right(i[1..].parse::<usize>().unwrap() / 90),
        'F' => match ferry.direction {
            North => ferry.y += i[1..].parse::<i32>().unwrap(),
            South => ferry.y -= i[1..].parse::<i32>().unwrap(),
            East => ferry.x += i[1..].parse::<i32>().unwrap(),
            West => ferry.x -= i[1..].parse::<i32>().unwrap(),
        },
        _ => panic!("unknown character {}", i),
    });
    println!("{}", ferry.x.abs() + ferry.y.abs());
}
