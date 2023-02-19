// day10 parse             time:   [2.1125 us 2.1386 us 2.1670 us]
// day10 part 1            time:   [848.92 ns 859.54 ns 872.40 ns]
// day10 part 2            time:   [1.1545 us 1.1669 us 1.1816 us]
use num_integer::binomial;

type Input = String;
type Parsed = Vec<u64>;

pub fn part1(oinput: &Parsed) -> usize {
    let mut input = oinput.clone();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);

    let mut diff3 = 0;
    let mut diff1 = 0;
    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];
        if diff == 3 {
            diff3 += 1;
        } else if diff == 1 {
            diff1 += 1;
        } else {
            unreachable!()
        }
    }
    diff3 * diff1
}

pub fn part2(oinput: &Parsed) -> usize {
    let mut input = oinput.clone();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);

    let mut diff1_sequence = 0;
    let mut combos = 1;
    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];
        if diff == 3 {
            if diff1_sequence > 1 {
                let removable = diff1_sequence - 1;
                // 1 is binomial(removable, 0) + removable is binomial(removable, 1)
                let mut prod = 1 + removable;
                if removable >= 2 {
                    prod += binomial(removable, 2);
                }
                combos *= prod;
            }
            diff1_sequence = 0;
        } else if diff == 1 {
            diff1_sequence += 1;
        } else {
            unreachable!()
        }
    }
    combos
}

pub fn parse(s: &Input) -> Parsed {
    s.lines().map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::get_input;

    #[test]
    fn test_day10() {
        let test_input = "16
10
15
5
1
11
7
19
6
12
4"
        .to_owned();
        assert_eq!(part1(&parse(&test_input)), 35);
        assert_eq!(part2(&parse(&test_input)), 8);

        let test_input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        .to_owned();
        assert_eq!(part2(&parse(&test_input)), 19208);
    }

    #[test]
    fn day10() {
        let input = get_input(2020, 10).unwrap();
        assert_eq!(part1(&parse(&input)), 2277);
        assert_eq!(part2(&parse(&input)), 37024595836928);
    }
}

// COOL SOLUTION I FOUND HERE: https://github.com/Ben-Lichtman/advent_of_code_2020/blob/master/src/day10.rs

// use std::fs::read_to_string;

// fn parse_to_vec(i: &str) -> Vec<u32> { i.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>() }

// fn main() {
// 	let input = read_to_string("input/day10/1.txt").unwrap();
// 	let mut nums = parse_to_vec(&input);
// 	nums.push(0);
// 	nums.sort_unstable();
// 	nums.push(nums[nums.len() - 1] + 3);

// 	let differences = nums.windows(2).map(|window| window[1] - window[0]).fold(
// 		(0, 0, 0),
// 		|(sum_1, sum_2, sum_3), new| match new {
// 			1 => (sum_1 + 1, sum_2, sum_3),
// 			2 => (sum_1, sum_2 + 1, sum_3),
// 			3 => (sum_1, sum_2, sum_3 + 1),
// 			_ => panic!("Invalid difference"),
// 		},
// 	);

// 	println!("Part 1: {}", differences.0 * differences.2);

// 	let mut paths = vec![0u64; nums.len()];
// 	paths[0] = 1;
// 	for i in 0..nums.len() {
// 		let current_num = nums[i];
// 		let current_paths = paths[i];

// 		for offset in 1..=3 {
// 			if let Some(&x) = nums.get(i + offset) {
// 				if x <= current_num + 3 {
// 					paths[i + offset] += current_paths;
// 				}
// 			}
// 		}
// 	}

// 	println!("Part 2: {}", paths.last().unwrap());
// }
