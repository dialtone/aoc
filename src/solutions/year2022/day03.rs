use itertools::Itertools;
use std::collections::HashSet;

type Parsed = Vec<(HashSet<char>, HashSet<char>)>;

pub fn part1(input: &Parsed) -> u64 {
    let mut tot = 0;

    for (comp1, comp2) in input {
        let r = comp1.intersection(comp2);
        tot += r.into_iter().map(|&c| priority(c)).sum::<u64>();
    }

    tot
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

pub fn parse1(s: &str) -> Parsed {
    let mut res = vec![];

    for line in s.lines() {
        let chars = line.trim().chars();
        let half = chars.count() / 2;

        let compart_1: HashSet<char> = line.chars().take(half).collect();
        let compart_2: HashSet<char> = line.chars().skip(half).take(half).collect();

        res.push((compart_1, compart_2));
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
        let parsed = parse1(input);
        assert_eq!(part1(&parsed), 157);
        assert_eq!(parse2(input), 12);
    }

    #[test]
    fn day03() {
        let input = get_input(2022, 3).unwrap();
        let parsed = parse1(&input);
        assert_eq!(part1(&parsed), 8394);
        assert_eq!(parse2(&input), 12);
    }
}
