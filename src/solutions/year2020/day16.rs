use super::*;

type Input = String;
type Parsed = String;

pub fn part1(input: &Parsed) -> usize {
    5
}

pub fn part2(input: &Parsed) -> usize {
    5
}

pub fn parse(s: &Input) -> &Parsed {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16() {
        let test_input = "".to_owned();
        assert_eq!(part1(&parse(&test_input)), 5);
        // assert_eq!(part2(&parse(&test_input)), 5);
    }

    // #[test]
    // fn day16() {
    //     let input = get_input(2020, 16).unwrap();
    //     assert_eq!(part1(&parse(&input)), 5);
    //     assert_eq!(part2(&parse(&input)), 5);
    // }
}
