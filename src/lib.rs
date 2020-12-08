pub mod day1;
pub mod day2;
pub mod day3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        day1::day1();
    }
    #[test]
    fn day2part1() {
        day2::part1();
    }

    #[test]
    fn day2part2() {
        day2::part2();
    }

    #[test]
    fn day3part1() {
        day3::part1();
    }

    #[test]
    fn day3part2() {
        day3::part2();
    }
}
