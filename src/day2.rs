use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?P<min>\d+)-(?P<max>\d+) (?P<ch>.): (?P<password>.*)"#)]
struct Policy {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

impl Policy {
    fn valid(&self) -> bool {
        (self.min..=self.max).contains(&self.password.chars().filter(|&x| x == self.ch).count())
    }

    fn valid_part2(&self) -> bool {
        let x = self.password.as_bytes()[self.min - 1] as char;
        let y = self.password.as_bytes()[self.max - 1] as char;
        (x == self.ch) ^ (y == self.ch)
    }
}

pub fn part1() {
    let result = include_str!("inputs/day2")
        .lines()
        .filter(|x| x.parse::<Policy>().unwrap().valid())
        .count();

    assert_eq!(result, 469);
}

pub fn part2() {
    let result = include_str!("inputs/day2")
        .lines()
        .filter(|x| x.parse::<Policy>().unwrap().valid_part2())
        .count();
    println!("{}", result);
}
