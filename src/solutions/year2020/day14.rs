// day14 parse             time:   [394.89 us 400.39 us 406.70 us]
// day14 part 1            time:   [19.102 us 19.443 us 19.879 us]
// day14 part 2            time:   [3.9798 ms 4.0443 ms 4.1136 ms]

use super::*;
use std::collections::HashMap;

type Input = String;
type Parsed = Vec<Operation>;

pub enum Operation {
    Mask(usize, usize, usize, Vec<usize>),
    Mem(usize, usize),
}

pub fn part1(input: &Parsed) -> usize {
    let mut mem = HashMap::new();

    let mut and_mask = 0usize;
    let mut or_mask = 0usize;
    for op in input {
        match op {
            &Operation::Mask(and_m, or_m, _, _) => {
                and_mask = and_m;
                or_mask = or_m;
            }
            &Operation::Mem(loc, val) => {
                let entry = mem.entry(loc).or_insert(0);
                let store_val = (val & and_mask) | or_mask;
                *entry = store_val;
            }
        }
    }
    mem.values().sum()
}

pub fn part2(input: &Parsed) -> usize {
    let mut mem = HashMap::new();

    let mut or_mask = 0usize;
    let mut and_float = 0usize;
    let mut float_masks = &Vec::new();
    for op in input {
        match op {
            Operation::Mask(_, or_m, and_fl, floats) => {
                float_masks = &floats;
                and_float = *and_fl;
                or_mask = *or_m;
            }
            Operation::Mem(loc, val) => {
                let decoded_loc = (loc | or_mask) & and_float;
                for mask in float_masks.iter() {
                    let entry = mem.entry(decoded_loc | mask).or_insert(0);
                    *entry = *val;
                }
            }
        }
    }
    mem.values().sum()
}

pub fn parse(s: &Input) -> Parsed {
    let mut program = Vec::new();
    for l in s.lines() {
        if l.starts_with("mask") {
            let m: &str = scan!("mask = {}" <- l).unwrap();
            let mut and_mask = 0usize;
            let mut or_mask = 0usize;
            let mut or_floats = Vec::new();
            let mut and_float = 0usize;
            for (i, c) in m.chars().enumerate() {
                match c {
                    '1' => {
                        and_mask = and_mask << 1;
                        or_mask = or_mask << 1 | 1;
                        and_float = and_float << 1 | 1;
                    }
                    '0' => {
                        and_mask = and_mask << 1;
                        or_mask = or_mask << 1;
                        and_float = and_float << 1 | 1;
                    }
                    'X' => {
                        and_mask = and_mask << 1 | 1;
                        or_mask = or_mask << 1;
                        and_float = and_float << 1;
                        or_floats.push(35 - i);
                    }
                    _ => unreachable!(),
                }
            }
            let mut float_masks = Vec::new();
            let n_floats = or_floats.len();
            for i in 0..(2usize).pow(n_floats as u32) {
                let mut float_mask = 0usize;
                for (j, float_id) in or_floats.iter().rev().enumerate() {
                    let bit_at = i & (1 << j) != 0;
                    if bit_at {
                        float_mask |= 1 << float_id;
                    } else {
                        float_mask |= 0 << float_id;
                    }
                }
                float_masks.push(float_mask);
            }
            program.push(Operation::Mask(and_mask, or_mask, and_float, float_masks));
        } else {
            let (loc, val) = scan!("mem[{}] = {}" <- l).unwrap();
            program.push(Operation::Mem(loc, val));
        }
    }
    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14() {
        let _test_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .to_owned();
        // assert_eq!(part1(&parse(&test_input)), 165);
        let test_input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .to_owned();
        assert_eq!(part2(&parse(&test_input)), 208);
    }

    #[test]
    fn day14() {
        let input = get_input(2020, 14).unwrap();
        assert_eq!(part1(&parse(&input)), 13556564111697);
        assert_eq!(part2(&parse(&input)), 4173715962894);
    }
}
