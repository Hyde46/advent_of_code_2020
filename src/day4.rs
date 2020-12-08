use std::collections::HashMap;

struct Passport {
    ecl: String,
    pid: usize,
    eyr: usize,
    hcl: String,
    iyr: usize,
    byr: usize,
    hgt: String,
}

pub fn part1() {
    let result = include_str!("inputs/day4")
        .split("\n\n")
        .filter(|entry| {
            vec!["ecl:", "pid:", "eyr:", "hcl:", "iyr:", "byr:", "hgt:"]
                .iter()
                .all(|&x| entry.contains(x))
        })
        .count();
    println!("{}", result);
}
