use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/4.txt").expect("Couldn't read the file");

    println!(
        "{}",
        input
            .split("\n\n")
            .filter(|i| i.contains("byr")
                && i.contains("iyr")
                && i.contains("eyr")
                && i.contains("hgt")
                && i.contains("hcl")
                && i.contains("ecl")
                && i.contains("pid"))
            .count()
    );
}
