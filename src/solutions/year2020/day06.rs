use std::collections::BTreeSet;
use std::iter::FromIterator;

// day6 parse              time:   [158.82 us 161.46 us 164.61 us]
pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|rows| {
            rows.lines()
                .map(|row| row.chars().fold(0u32, |acc, c| acc | 1 << (c as u8 - b'a')))
                .collect()
        })
        .collect()
}

// time:   [57.423 us 58.066 us 58.848 us]
pub fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|rows| {
            rows.split_whitespace()
                .flat_map(|s| s.chars())
                .fold(0u32, |acc, c| acc | 1 << (c as u8 - b'a'))
                .count_ones()
        })
        .sum()
}
//day6 part 1             time:   [1.3621 us 1.3807 us 1.4033 us]
// pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
//     input
//         .iter()
//         .map(|group| group.iter().fold(0, |acc, n| acc | n).count_ones())
//         .sum()
// }

// time:   [62.741 us 63.511 us 64.555 us]
// pub fn part2(input: &String) -> u32 {
//     input
//         .split("\n\n")
//         .map(|rows| {
//             rows.lines()
//                 .map(|a| a.chars().fold(0u32, |acc, c| acc | 1 << (c as u8 - b'a')))
//                 .fold(!0u32, |acc, c| acc & c).count_ones()
//         })
//         .sum()
// }

// day6 part 2             time:   [1.1820 us 1.1929 us 1.2050 us]
// pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
//     input
//         .iter()
//         .map(|group| {
//             group
//                 .iter()
//                 .fold(u32::max_value(), |acc, n| acc & n)
//                 .count_ones()
//         })
//         .sum()
// }

// Pre-optimization pass
// day6 part 1             time:   [656.05 us 662.19 us 670.25 us]
// pub fn part1(input: &String) -> usize {
//     input
//         .split("\n\n")
//         .map(|answers| BTreeSet::from(answers.replace(&"\n", &"").chars().collect()).len())
//         .sum()
// }

// day6 part 2             time:   [1.2361 ms 1.2543 ms 1.2765 ms]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|answers| {
            answers
                .lines()
                .fold(BTreeSet::from_iter('a'..='z'), |acc, row| {
                    acc.intersection(&row.chars().collect()).cloned().collect()
                })
                .len()
            // let sets = answers
            //     .lines()
            //     .map(|row| BTreeSet::from(row.chars().collect()))
            //     .collect::<Vec<BTreeSet<char>>>();
            // sets[0]
            //     .iter()
            //     .filter(|k| sets[1..].iter().all(|s| s.contains(k)))
            //     .collect::<Vec<&char>>()
            //     .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::get_input;

    #[test]
    fn test_day06() {
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
        assert_eq!(part2(&test_input), 6);
    }

    #[test]
    fn day06() {
        let input = get_input(2020, 6).unwrap();
        assert_eq!(part1(&input), 6170);
        assert_eq!(part2(&input), 2947);
    }
}
