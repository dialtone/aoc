// day14b parse            time:   [112.03 us 113.19 us 114.67 us]
// day14b part 1           time:   [30.978 us 31.465 us 32.274 us]
// day14b part 2           time:   [4.0520 ms 4.0855 ms 4.1223 ms]

use super::*;
use std::collections::HashMap;

type Input = String;
type Parsed<'a> = Vec<Operation<'a>>;

pub enum Operation<'a> {
    Mask(&'a str),
    Mem(usize, usize),
}

pub fn part1(input: &Parsed) -> usize {
    let mut mem = HashMap::new();

    let mut and_mask = 0usize;
    let mut or_mask = 0usize;

    for op in input {
        match op {
            &Operation::Mask(mask) => {
                and_mask = 0;
                or_mask = 0;
                for c in mask.chars() {
                    match c {
                        '1' => {
                            and_mask = and_mask << 1;
                            or_mask = or_mask << 1 | 1;
                        }
                        '0' => {
                            and_mask = and_mask << 1;
                            or_mask = or_mask << 1;
                        }
                        'X' => {
                            and_mask = and_mask << 1 | 1;
                            or_mask = or_mask << 1;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            &Operation::Mem(loc, val) => {
                let entry = mem.entry(loc).or_insert(0);
                let store_val = val & and_mask | or_mask;
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
    let mut float_masks = Vec::new();

    for op in input {
        match *op {
            Operation::Mask(mask) => {
                or_mask = 0;
                // float_masks = Vec::new();
                let mut or_floats = Vec::new();
                for (i, c) in mask.chars().enumerate() {
                    match c {
                        '1' => {
                            or_mask = or_mask << 1 | 1;
                            and_float = and_float << 1 | 1;
                        }
                        '0' => {
                            or_mask <<= 1;
                            and_float = and_float << 1 | 1;
                        }
                        'X' => {
                            or_mask <<= 1;
                            and_float <<= 1;
                            or_floats.push(35 - i);
                        }
                        _ => unreachable!(),
                    }
                }
                float_masks = Vec::new();
                let n_floats = or_floats.len();
                for i in 0..(2usize).pow(n_floats as u32) {
                    let mut float_mask = 0usize;
                    for (j, float_id) in or_floats.iter().rev().enumerate() {
                        float_mask |= (i & (1 << j)) << (float_id - j);
                    }
                    float_masks.push(float_mask);
                }
            }
            Operation::Mem(loc, val) => {
                let decoded_loc = (loc | or_mask) & and_float;
                // neat trick with xor for the combinations.
                // let index = index | current_or_mask;
                //     let mut xor = current_x_mask + 1;

                //     for _ in 0..2_u16.pow(current_x_mask.count_ones()) {
                //         xor = (xor - 1) & current_x_mask;
                //         *memory.entry(index ^ xor).or_default() = num;
                //     }
                // }
                for mask in float_masks.iter() {
                    let entry = mem.entry(decoded_loc | mask).or_insert(0);
                    *entry = val;
                }
            }
        }
    }
    mem.values().sum()
}

pub fn parse<'a>(s: &'a Input) -> Parsed<'a> {
    let mut program = Vec::new();
    for l in s.lines() {
        if l.starts_with("mask") {
            let m: &str = scan!("mask = {}" <- l).unwrap();
            program.push(Operation::Mask(m));
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
    fn test_day14b() {
        let test_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .to_owned();
        assert_eq!(part1(&parse(&test_input)), 165);
        let test_input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .to_owned();
        assert_eq!(part2(&parse(&test_input)), 208);
    }

    #[test]
    fn day14b() {
        let input = get_input(2020, 14).unwrap();
        assert_eq!(part1(&parse(&input)), 13556564111697);
        assert_eq!(part2(&parse(&input)), 4173715962894);
    }
}
