pub fn part1() {
    let map_width = 31;
    let slope_x = 3;

    let result = include_str!("inputs/day3")
        .lines()
        .enumerate()
        .filter(|(pos, e)| e.chars().nth((pos * slope_x) % map_width).unwrap() == '#')
        .count();
    assert_eq!(result, 274);
}

pub fn part2() {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let map_width = 31;

    let product = slopes
        .iter()
        .map(|(slope_x, slope_y)| {
            //Previous solution, but enhanced to utilize slope_y
            include_str!("inputs/day3")
                .lines()
                .enumerate()
                .filter(|(pos, _)| pos % slope_y == 0)
                .map(|(_, e)| e)
                .enumerate()
                .filter(|(pos, e)| e.chars().nth((pos * slope_x) % map_width).unwrap() == '#')
                .count()
        })
        .fold(1, |mult, x| mult * x);
    assert_eq!(product, 6050183040);
}
