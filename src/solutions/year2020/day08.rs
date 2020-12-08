// day8 parse              time:   [139.97 us 143.28 us 147.14 us]
// Found 8 outliers among 100 measurements (8.00%)
//   4 (4.00%) high mild
//   4 (4.00%) high severe

// day8 part 1             time:   [13.944 us 14.209 us 14.546 us]
// Found 12 outliers among 100 measurements (12.00%)
//   2 (2.00%) high mild
//   10 (10.00%) high severe

// day8 part 2             time:   [2.1391 ms 2.1801 ms 2.2241 ms]
// Found 7 outliers among 100 measurements (7.00%)
//   7 (7.00%) high mild

use super::*;
use std::collections::BTreeSet;

pub fn part1(input: &Vec<(&str, isize)>) -> isize {
    let (broken, acc) = compute(input);
    acc
}

pub fn compute(input: &Vec<(&str, isize)>) -> (bool, isize) {
    let mut pc: isize = 0;
    let mut acc = 0;
    let mut seen = BTreeSet::new();
    while (pc as usize) < input.len() {
        let instruction = input[pc as usize];
        if seen.contains(&pc) {
            return (true, acc);
        }
        seen.insert(pc);
        match instruction {
            ("nop", _value) => pc += 1,
            ("acc", value) => {
                acc += value;
                pc += 1
            }
            ("jmp", value) => pc += value,
            _ => {
                println!("error");
            }
        }
    }
    (false, acc)
}

pub fn part2(input: &Vec<(&str, isize)>) -> isize {
    loop {
        for (i, (instruction, _)) in input.iter().enumerate() {
            if instruction == &"jmp" || instruction == &"nop" {
                let mut fixed_input = input.clone();
                let new_instruction = if instruction == &"jmp" { "nop" } else { "jmp" };
                fixed_input[i] = (new_instruction, fixed_input[i].1);
                let (broken, acc) = compute(&fixed_input);
                if !broken {
                    return acc;
                }
            }
        }
    }
}

pub fn parse(s: &String) -> Vec<(&str, isize)> {
    let mut program = Vec::new();
    for line in s.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        program.push((instruction[0], instruction[1].parse().unwrap()))
    }
    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08() {
        let test_input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .to_owned();
        assert_eq!(part1(&parse(&test_input)), 5);
        assert_eq!(part2(&parse(&test_input)), 8);
    }

    #[test]
    fn day08() {
        let input = get_input(2020, 8).unwrap();
        assert_eq!(part1(&parse(&input)), 2058);
        assert_eq!(part2(&parse(&input)), 1000);
    }
}
