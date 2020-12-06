use super::*;
use std::collections::BTreeSet;

pub fn part1(input: &String) -> usize {
    input
        .split("\n\n")
        .map(|answers| BTreeSet::from(answers.replace(&"\n", &"").chars().collect()).len())
        .sum()
}

pub fn part2(input: &String) -> usize {
    input
        .split("\n\n")
        .map(|answers| {
            let sets = answers
                .lines()
                .map(|row| BTreeSet::from(row.chars().collect()))
                .collect::<Vec<BTreeSet<char>>>();
            sets[0]
                .iter()
                .filter(|k| sets[1..].iter().all(|s| s.contains(k)))
                .collect::<Vec<&char>>()
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02() {
        let test_input = "abc

a
b
c

ab
ac

a
a
a
a

b"
        .to_owned();
        assert_eq!(part1(&test_input), 11);
        assert_eq!(part2(&&test_input), 6);
    }

    #[test]
    fn day02() {
        let input = get_input(2020, 6).unwrap();
        assert_eq!(part1(&input), 6170);
        assert_eq!(part2(&input), 2947);
    }
}
