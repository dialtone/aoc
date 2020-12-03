use super::*;

pub fn part1(input: &String) {
}

pub fn part2(input: &String) {
}

pub fn parse(s: &String) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_test() {
        let test_input = "".to_owned();
        assert_eq!(part1(&test_input), 7);
        assert_eq!(part2(&test_input), 336);
    }

    #[test]
    fn day02() {
        let input = get_input(2020, 3).unwrap();
        assert_eq!(part1(&input), 145);
        assert_eq!(part2(&input), 3424528800);
    }
}

