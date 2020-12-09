// day9 parse              time:   [21.628 us 22.192 us 22.861 us]
// Found 13 outliers among 100 measurements (13.00%)
//   4 (4.00%) high mild
//   9 (9.00%) high severe

// day9 part 1             time:   [28.621 us 29.204 us 29.954 us]
// Found 9 outliers among 100 measurements (9.00%)
//   3 (3.00%) high mild
//   6 (6.00%) high severe

// day9 part 2             time:   [3.0866 ms 3.1506 ms 3.2261 ms]
// Found 14 outliers among 100 measurements (14.00%)
//   6 (6.00%) high mild
//   8 (8.00%) high severe

use super::*;

type Input = String;
type Parsed = Vec<usize>;

pub fn part1(input: &Parsed, preamble: usize) -> usize {
    let mut start = 0;
    for i in 0..input.len() {
        if i <= preamble {
            continue;
        }
        let num = input[i];
        let mut found = false;
        for other_num in input[start..i].iter() {
            if num < *other_num {
                continue;
            }
            if input[start..i].contains(&(num - other_num)) {
                found = true;
                break;
            }
        }
        if !found {
            return num;
        }

        start += 1;
    }
    0
}

pub fn part2(input: &Parsed, p1: usize) -> usize {
    for i in 0..input.len() {
        let mut running_sum = 0;
        let mut stride = 2;
        while running_sum <= p1 {
            running_sum = input[i..i + stride].iter().sum();
            if running_sum == p1 {
                let mut sorted = input[i..i + stride].iter().collect::<Vec<&usize>>();
                sorted.sort();
                return sorted[0] + sorted[stride - 1];
            }
            stride += 1;
        }
    }
    0
}

pub fn parse(s: &Input) -> Parsed {
    s.lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09() {
        let test_input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
        .to_owned();
        assert_eq!(part1(&parse(&test_input), 5), 127);
        assert_eq!(part2(&parse(&test_input), 127), 62);
    }

    #[test]
    fn day09() {
        let input = get_input(2020, 9).unwrap();
        assert_eq!(part1(&parse(&input), 25), 3199139634);
        assert_eq!(part2(&parse(&input), 3199139634), 438559930);
    }
}
