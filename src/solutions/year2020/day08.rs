// day8 parse              time:   [21.228 us 21.599 us 22.025 us]
//                         change: [-1.7038% +1.2016% +4.5279%] (p = 0.45 > 0.05)
//                         No change in performance detected.
// Found 13 outliers among 100 measurements (13.00%)
//   7 (7.00%) high mild
//   6 (6.00%) high severe

// day8 part 1             time:   [637.10 ns 651.10 ns 667.99 ns]
//                         change: [-0.3924% +1.2606% +3.0887%] (p = 0.20 > 0.05)
//                         No change in performance detected.
// Found 13 outliers among 100 measurements (13.00%)
//   4 (4.00%) high mild
//   9 (9.00%) high severe

// day8 part 2             time:   [11.444 us 11.503 us 11.571 us]
//                         change: [-91.669% -91.490% -91.317%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 9 outliers among 100 measurements (9.00%)
//   2 (2.00%) low mild
//   4 (4.00%) high mild
//   3 (3.00%) high severe

use super::*;

pub fn part1(input: &Vec<(&str, isize)>) -> isize {
    let (_, acc) = compute(input);
    acc
}

pub fn compute(input: &Vec<(&str, isize)>) -> (bool, isize) {
    let mut pc: isize = 0;
    let mut acc = 0;
    let mut seen = vec![false; 4096];
    while (pc as usize) < input.len() {
        let instruction = input[pc as usize];
        if seen[pc as usize] {
            return (true, acc);
        }
        seen[pc as usize] = true;
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

#[inline]
fn swap_instruction(input: (&str, isize)) -> (&str, isize) {
    let new_instruction = if input.0 == "jmp" { "nop" } else { "jmp" };
    (new_instruction, input.1)
}

pub fn part2(oinput: &Vec<(&str, isize)>) -> isize {
    let mut input = oinput.clone();
    loop {
        let mut last_changed = 600000;
        for i in 0..=input.len() {
            let (instruction, _) = input[i];
            if last_changed != 600000 {
                let new_instruction = swap_instruction(input[last_changed]);
                input[last_changed] = new_instruction;
            }
            if instruction == "jmp" || instruction == "nop" {
                last_changed = i;
                input[i] = swap_instruction(input[i]);
                let (broken, acc) = compute(&input);
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
        // If you collect here you'll make this almost 10x slower
        let mut instruction = line.split_whitespace();
        program.push((
            instruction.next().unwrap(),
            instruction.next().unwrap().parse().unwrap(),
        ))
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
