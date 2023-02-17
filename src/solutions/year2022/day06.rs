use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

// year 22 day06 part 1    time:   [1.4595 µs 1.4607 µs 1.4622 µs]
pub fn part1(input: &str) -> usize {
    for (i, (a, b, c, d)) in input.chars().tuple_windows::<(_, _, _, _)>().enumerate() {
        if a != b && b != c && c != d && a != c && a != d && b != d {
            return i + 4;
        }
    }
    0
}

// year 22 day06 part 2    time:   [18.139 µs 18.239 µs 18.344 µs]
pub fn part2(input: &str) -> usize {
    let input = input.chars().collect::<Vec<char>>();
    for (i, message) in input.windows(14).enumerate() {
        if (0..14)
            .tuple_combinations()
            .all(|(a, b)| message[a] != message[b])
        {
            return i + 14;
        }
    }
    0
}

// year 22 day06 part 2b   time:   [693.60 µs 695.08 µs 696.84 µs]
pub fn part2b(input: &str) -> usize {
    let input = input.chars().collect::<Vec<char>>();
    for (i, message) in input.windows(14).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(message);
        if set.len() == 14 {
            return i + 14;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day06_test() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part1(input), 7);
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(input), 5);
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part1(input), 6);
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part1(input), 10);
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part1(input), 11);

        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part2(input), 19);
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part2(input), 23);
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part2(input), 23);
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part2(input), 29);
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part2(input), 26);
    }

    #[test]
    fn day05() {
        let input = get_input(2022, 6).unwrap();
        assert_eq!(part1(&input), 1134);
        assert_eq!(part2(&input), 2263);
        assert_eq!(part2b(&input), 2263);
    }
}
