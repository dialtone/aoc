use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let half = l.chars().count() / 2;

            let c1: HashSet<char> = l.chars().take(half).collect();
            let c2: HashSet<char> = l.chars().skip(half).collect();
            c1.intersection(&c2).map(|&c| priority(c)).sum::<u64>()
        })
        .sum()
}

pub fn priority(c: char) -> u64 {
    match c {
        'a'..='z' => (c as u8 - b'a' + 1).into(),
        'A'..='Z' => (c as u8 - b'A' + 27).into(),
        _ => unreachable!(),
    }
}

pub fn parse2(s: &str) -> u64 {
    let mut res = 0;
    for group in &s.lines().chunks(3) {
        let sets: Vec<_> = group
            .map(|l| l.chars().collect::<HashSet<char>>())
            .collect();

        let mut iter = sets.into_iter();
        res += iter
            .next()
            .map(|set| {
                iter.fold(set, |set1, set2| {
                    set1.intersection(&set2).cloned().collect()
                })
            })
            .map(|s| s.iter().map(|&x| priority(x)).sum::<u64>())
            .unwrap_or(0);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day03_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(part1(input), 157);
        assert_eq!(parse2(input), 70);
    }

    #[test]
    fn day03() {
        let input = get_input(2022, 3).unwrap();
        assert_eq!(part1(&input), 8394);
        assert_eq!(parse2(&input), 2413);
    }
}
