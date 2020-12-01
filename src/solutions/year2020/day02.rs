use super::*;

pub fn part1(input: &Vec<u32>) {}

pub fn part2(input: &Vec<u32>) {}

pub fn parse(s: String) -> Vec<u32> {
    s.trim()
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_test() {
        let expense_report = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&expense_report), 514579);
        assert_eq!(part2(&expense_report), 241861950);
    }

    #[test]
    fn day02() {
        let input = get_input(2020, 1).unwrap();
        let expense_report = parse(input);
        assert_eq!(part1(&expense_report), 876459);
        assert_eq!(part2(&expense_report), 116168640);
    }
}
