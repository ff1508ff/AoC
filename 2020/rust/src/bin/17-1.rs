use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/17.txt").expect("Couldn't read the file");

    let mut pocket_dimension: [[[bool; 30]; 30]; 30] = [[[false; 30]; 30]; 30];
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                pocket_dimension[15][y + 10][x + 10] = true;
            }
        })
    });

    for _ in 1..=6 {
        let old = pocket_dimension;
        for z in 0..30 {
            for y in 0..30 {
                for x in 0..30 {
                    let nambors = count_nabors(&old, z, y, x);
                    if nambors == 3 {
                        pocket_dimension[z][y][x] = true;
                    } else if nambors != 2 {
                        pocket_dimension[z][y][x] = false;
                    }
                }
            }
        }
    }
    println!(
        "{:?}",
        pocket_dimension
            .iter()
            .flatten()
            .flatten()
            .filter(|&&c| c)
            .count()
    );
}

fn count_nabors(pocket_dimension: &[[[bool; 30]; 30]; 30], z: usize, y: usize, x: usize) -> usize {
    let mut count = 0;
    for a in -1..=1 {
        for b in -1..=1 {
            for c in -1..=1 {
                if a == 0 && b == 0 && c == 0 {
                    continue;
                }
                let z = z as isize + a;
                let y = y as isize + b;
                let x = x as isize + c;
                if (0..30).contains(&z) && (0..30).contains(&y) && (0..30).contains(&x) {
                    if pocket_dimension[z as usize][y as usize][x as usize] {
                        count += 1;
                        if count == 4 {
                            return count;
                        }
                    }
                }
            }
        }
    }
    count
}
