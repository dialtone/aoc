// use crate::solutions::clear_screen;
// use crate::solutions::g#![feature(portable_simd)]o_to_top;
use std::simd::u64x2;

pub struct Monkey {
    pub stack: Vec<u64>,
    pub op: Op,
    pub div: u64x2,
    pub conditions: (usize, usize),
}

#[derive(Clone, Copy)]
pub enum Op {
    Square,
    Add(u64x2),
    Mul(u64x2),
}

pub fn parse(input: &[u8]) -> Vec<Monkey> {
    let mut monkeys = vec![];

    let mut input = input.split(|c| c == &b'\n');
    while let Some(mut line) = input.next() {
        if line.is_empty() {
            // end monkey
            continue;
        }

        // monkey name skip
        line = input.next().unwrap();

        // items
        let stack = line[line.iter().position(|c| c == &b':').unwrap() + 1..]
            .split(|c| c == &b',')
            .map(|it| if it[0] == b' ' { &it[1..] } else { it })
            .map(|it| atoi::atoi(it).unwrap())
            .collect();

        // ops
        line = input.next().unwrap();
        let mut op_and_num = line[23..].split(|c| c == &b' ');
        let op = op_and_num.next().unwrap();
        let num = op_and_num.next().unwrap();
        let operation: Op = match (op[0], num) {
            (b'*', b"old") => Op::Square,
            (b'*', n) => Op::Mul(u64x2::splat(atoi::atoi(n).unwrap())),
            (b'+', n) => Op::Add(u64x2::splat(atoi::atoi(n).unwrap())),
            _ => {
                unreachable!();
            }
        };

        // divisible test
        line = input.next().unwrap();
        let divisible = u64x2::splat(atoi::atoi(&line[21..]).unwrap());

        // true condition
        line = input.next().unwrap();
        let condition = atoi::atoi(&line[29..]).unwrap();

        // false condition
        line = input.next().unwrap();
        let conditions = (condition, atoi::atoi(&line[30..]).unwrap());

        monkeys.push(Monkey {
            stack,
            op: operation,
            div: divisible,
            conditions,
        });
    }

    monkeys
}

// year 22 day11 part 1    time:   [4.7589 µs 4.7724 µs 4.7906 µs]
pub fn part1(input: &[u8]) -> usize {
    let mut monkeys = parse(input);
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let len = monkeys[i].stack.len();
            for j in 0..len {
                let item = monkeys[i].stack[j];
                inspections[i] += 1;
                let new_item = match monkeys[i].op {
                    Op::Square => item * item / 3,
                    Op::Mul(n) => item * n[0] / 3,
                    Op::Add(n) => (item + n[0]) / 3,
                };
                let throw_to = if new_item % monkeys[i].div[0] == 0 {
                    monkeys[i].conditions.0
                } else {
                    monkeys[i].conditions.1
                };
                monkeys[throw_to].stack.push(new_item);
            }
            monkeys[i].stack.clear();
        }
    }
    inspections.sort();
    inspections.iter().rev().take(2).product::<usize>()
}

// year 22 day11 part 2    time:   [1.4882 ms 1.4896 ms 1.4911 ms]
pub fn part2(input: &[u8]) -> usize {
    let monkeys = parse(input);

    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    let mut items = [[0u64; 50]; 8];
    // pointer to last item in items for each monkey
    let mut m_cnt = [0usize; 8];

    let mcm = monkeys.iter().map(|m| m.div[0]).product::<u64>();
    for (n, m) in monkeys.iter().enumerate() {
        for i in m.stack.iter() {
            items[n][m_cnt[n]] = *i;
            m_cnt[n] += 1;
        }
    }
    let mcm = u64x2::splat(mcm);

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            match monkeys[i].op {
                Op::Square => {
                    let n = u64x2::splat(1);
                    for v in (0..m_cnt[i]).step_by(n.lanes()) {
                        let val = u64x2::from_slice(&items[i][v..v + 2]);
                        let new_item = val * val % mcm;

                        let new_rem = new_item % monkeys[i].div;
                        let throw_to = if new_rem[0] == 0 {
                            monkeys[i].conditions.0
                        } else {
                            monkeys[i].conditions.1
                        };

                        let throw_to1 = if new_rem[0] == 0 {
                            monkeys[i].conditions.0
                        } else {
                            monkeys[i].conditions.1
                        };

                        inspections[i] += 1;
                        items[throw_to][m_cnt[throw_to]] = new_item[0];
                        m_cnt[throw_to] += 1;
                        if v + 1 < m_cnt[i] {
                            inspections[i] += 1;
                            items[throw_to1][m_cnt[throw_to1]] = new_item[1];
                            m_cnt[throw_to1] += 1;
                        }
                    }
                    m_cnt[i] = 0;
                }
                Op::Mul(n) => {
                    for v in (0..m_cnt[i]).step_by(n.lanes()) {
                        let val = u64x2::from_slice(&items[i][v..v + 2]);
                        let new_item = val * n % mcm;

                        let new_rem = new_item % monkeys[i].div;
                        let throw_to = if new_rem[0] == 0 {
                            monkeys[i].conditions.0
                        } else {
                            monkeys[i].conditions.1
                        };

                        let throw_to1 = if new_rem[0] == 0 {
                            monkeys[i].conditions.0
                        } else {
                            monkeys[i].conditions.1
                        };

                        inspections[i] += 1;
                        items[throw_to][m_cnt[throw_to]] = new_item[0];
                        m_cnt[throw_to] += 1;

                        if v + 1 < m_cnt[i] {
                            inspections[i] += 1;
                            items[throw_to1][m_cnt[throw_to1]] = new_item[1];
                            m_cnt[throw_to1] += 1;
                        }
                    }
                    m_cnt[i] = 0;
                }
                Op::Add(n) => {
                    for v in (0..m_cnt[i]).step_by(n.lanes()) {
                        let val = u64x2::from_slice(&items[i][v..v + 2]);
                        let new_item = val + n % mcm;

                        let new_rem = new_item % monkeys[i].div;
                        let throw_to = if new_rem[0] == 0 {
                            monkeys[i].conditions.0
                        } else {
                            monkeys[i].conditions.1
                        };

                        let throw_to1 = if new_rem[0] == 0 {
                            monkeys[i].conditions.0
                        } else {
                            monkeys[i].conditions.1
                        };

                        inspections[i] += 1;
                        items[throw_to][m_cnt[throw_to]] = new_item[0];
                        m_cnt[throw_to] += 1;

                        if v + 1 < m_cnt[i] {
                            inspections[i] += 1;
                            items[throw_to1][m_cnt[throw_to1]] = new_item[1];
                            m_cnt[throw_to1] += 1;
                        }
                    }
                    m_cnt[i] = 0;
                }
            };
        }
    }
    inspections.sort();
    inspections.iter().rev().take(2).product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn test_example() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        // assert_eq!(part1(input.as_bytes()), 10605);
        assert_eq!(part2(input.as_bytes()), 2713310158);
    }

    // #[test]
    // fn test() {
    //     let input = get_input(2022, 11).unwrap();
    //     assert_eq!(part1(input.as_bytes()), 58786);
    //     assert_eq!(part2(input.as_bytes()), 14952185856);
    // }
}
