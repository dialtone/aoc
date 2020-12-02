use super::*;
use std::iter::Iterator;

pub fn part1(input: &String) -> usize {
    // time:   [6.6843 us 6.8296 us 6.9876 us] without parsing
    parse(input)
        .filter(|row| (row.0..=row.1).contains(&bytecount::count(row.3, row.2 as u8)))
        .count()

    // time:   [71.867 us 73.218 us 74.751 us] without parsing
    // let mut valid = 0;
    // for row in input {
    //     let v = (row.3).matches(&row.2).count();
    //     if v >= row.0 && v <= row.1 {
    //         valid += 1;
    //     }
    // }
    // return valid;
}

pub fn part2(input: &String) -> usize {
    // time:   [1.9534 us 1.9681 us 1.9845 us] without parsing
    parse(input)
        .filter(|row| (row.3[row.0 - 1] == row.2 as u8) != (row.3[row.1 - 1] == row.2 as u8))
        .count()

    // time:   [31.714 us 32.140 us 32.650 us] without parsing
    // let mut valid = 0;
    // for row in input {
    //     let sentinel = row.2.chars().nth(0).unwrap();
    //     let test1 = row.3.chars().nth(row.0 - 1).unwrap() == sentinel;
    //     let test2 = row.3.chars().nth(row.1 - 1).unwrap() == sentinel;
    //     if (test1 && !test2) || (!test1 && test2) {
    //         valid += 1;
    //     }
    // }
    // return valid;
}

pub fn parse(s: &String) -> impl Iterator<Item = (usize, usize, char, &[u8])> {
    s.trim()
        .lines()
        .map(|x| scan!("{}-{} {}: {}" <- x).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_test() {
        let test_input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            .to_owned();
        assert_eq!(part1(&test_input), 2);
        assert_eq!(part2(&test_input), 1);
    }

    #[test]
    fn day02() {
        let input = get_input(2020, 2).unwrap();
        assert_eq!(part1(&input), 422);
        assert_eq!(part2(&input), 451);
    }
}
