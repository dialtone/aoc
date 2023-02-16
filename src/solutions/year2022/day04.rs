pub fn part1(input: &str) -> u64 {
    let groups = parse(input);
    let mut full_overlap = 0;
    for (a, b) in groups {
        if (a & b) == a || (a & b) == b {
            full_overlap += 1;
        }
    }
    full_overlap
}

pub fn part2(input: &str) -> u64 {
    let groups = parse(input);
    let mut full_overlap = 0;
    for (a, b) in groups {
        if (a & b) != 0 {
            full_overlap += 1;
        }
    }
    full_overlap
}

fn parse(input: &str) -> Vec<(u128, u128)> {
    let mut res = vec![];
    for line in input.lines() {
        if let Some((elf1, elf2)) = line.split_once(',') {
            res.push((parse_range_into_u128(elf1), parse_range_into_u128(elf2)));
        }
    }
    res
}

pub fn parse_range_into_u128(s: &str) -> u128 {
    let (start, end) = s
        .split_once('-')
        .map(|(start, end)| (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap()))
        .unwrap();

    let mut i: u128 = 0;
    for shift in start..=end {
        i |= 1 << shift;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day04_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(part1(input), 2);
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn day04() {
        let input = get_input(2022, 4).unwrap();
        assert_eq!(part1(&input), 644);
        assert_eq!(part2(&input), 926);
    }
}
