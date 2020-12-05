use super::*;

pub fn part1(input: &String) -> usize {
    input
        .lines()
        .map(|row| {
            row.chars().fold(0, |acc, x| match x {
                'F' | 'L' => acc << 1,
                'B' | 'R' => acc << 1 | 1,
                _ => acc,
            })
        })
        .max()
        .unwrap()
}

pub fn part2(input: &String) -> usize {
    let mut seats = input
        .lines()
        .map(|row| {
            row.chars().fold(0, |acc, x| match x {
                'F' | 'L' => acc << 1,
                'B' | 'R' => acc << 1 | 1,
                _ => acc,
            })
        })
        .collect::<Vec<usize>>();
    seats.sort();
    let mut last = seats[0] - 1;
    for seat in seats {
        if seat - 1 != last {
            return seat - 1;
        }
        last = seat;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05() {
        let test_input = "FBFBBFFRLR".to_owned();
        assert_eq!(part1(&test_input), 357);
        let test_input = "BFFFBBFRRR".to_owned();
        assert_eq!(part1(&test_input), 567);
        let test_input = "FFFBBBFRRR".to_owned();
        assert_eq!(part1(&test_input), 119);
        let test_input = "BBFFBBFRLL".to_owned();
        assert_eq!(part1(&test_input), 820);
    }

    #[test]
    fn day05() {
        let input = get_input(2020, 5).unwrap();
        assert_eq!(part1(&input), 922);
        assert_eq!(part2(&input), 747);
    }
}
