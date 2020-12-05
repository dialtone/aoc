use super::*;

pub fn part1(input: &String) {}

pub fn part2(input: &String) {}

pub fn parse(s: &String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let test_input = "".to_owned();
        assert_eq!(part1(&parse(&test_input)));
        assert_eq!(part2(&parse(&test_input)));
    }

    #[test]
    fn day02() {
        let input = get_input(2020, 5).unwrap();
        assert_eq!(part1(&parse(&input)));
        assert_eq!(part2(&parse(&input)));
    }
}
