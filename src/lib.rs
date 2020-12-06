pub mod day1;
pub mod day2;

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
}