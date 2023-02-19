use std::collections::VecDeque;

#[derive(Debug)]
pub struct Monkey {
    pub stack: VecDeque<usize>,
    pub op: (usize, fn(usize, usize) -> usize),
    pub div: usize,
    pub conditions: (usize, usize),
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
            .collect::<VecDeque<usize>>();

        // ops
        line = input.next().unwrap();
        let mut op_and_num = line[23..].split(|c| c == &b' ');
        let op = op_and_num.next().unwrap();
        let num = op_and_num.next().unwrap();
        let operation: (usize, fn(usize, usize) -> usize) = match (op[0], num) {
            (b'*', b"old") => (1, |nn, old| nn * old * old),
            (b'*', n) => {
                let no = atoi::atoi(n).unwrap();
                (no, |nn, old| nn * old)
            }
            (b'+', n) => {
                let no = atoi::atoi(n).unwrap();
                (no, |nn, old| nn + old)
            }
            _ => {
                unreachable!();
            }
        };

        // divisible test
        line = input.next().unwrap();
        let divisible = atoi::atoi(&line[21..]).unwrap();

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

// year 22 day11 part 1    time:   [6.3379 µs 6.3534 µs 6.3709 µs]
pub fn part1(input: &[u8]) -> usize {
    let mut monkeys = parse(input);
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].stack.pop_front() {
                inspections[i] += 1;
                let new_item = monkeys[i].op.1(monkeys[i].op.0, item) / 3;
                let throw_to = if new_item % monkeys[i].div == 0 {
                    monkeys[i].conditions.0
                } else {
                    monkeys[i].conditions.1
                };
                monkeys[throw_to].stack.push_back(new_item);
            }
        }
    }
    inspections.sort();
    inspections.iter().rev().take(2).product::<usize>()
}

// year 22 day11 part 2    time:   [2.7470 ms 2.7531 ms 2.7610 ms]
pub fn part2(input: &[u8]) -> usize {
    let mut monkeys = parse(input);
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];

    let mcm = monkeys.iter().map(|m| m.div).product::<usize>();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].stack.pop_front() {
                inspections[i] += 1;
                let new_item = monkeys[i].op.1(monkeys[i].op.0, item) % mcm;
                let throw_to = if new_item % monkeys[i].div == 0 {
                    monkeys[i].conditions.0
                } else {
                    monkeys[i].conditions.1
                };
                monkeys[throw_to].stack.push_back(new_item);
            }
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
    fn day11_test() {
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
        assert_eq!(part1(input.as_bytes()), 10605);
        assert_eq!(part2(input.as_bytes()), 2713310158);
    }

    #[test]
    fn day11() {
        let input = get_input(2022, 11).unwrap();
        assert_eq!(part1(input.as_bytes()), 58786);
        assert_eq!(part2(input.as_bytes()), 14952185856);
    }
}

// pub fn parse(input: &[u8]) -> Vec<Monkey> {
//     let mut monkeys = vec![];
//
//     let mut stack: Vec<usize> = vec![];
//     let mut operation: (usize, fn(usize, usize) -> usize) = (0, |a, _| a);
//     let mut divisible = 0;
//     let mut conditions: (usize, usize) = (0, 0);
//     let mut condition = 0;
//
//     for line in input.split(|c| c == &b'\n') {
//         if line.is_empty() {
//             // finish_monkey
//             continue;
//         }
//
//         if &line[..4] == b"Monk" {
//             // new monkey
//             continue;
//         }
//
//         if &line[..4] == b"  St" {
//             stack = line[line.iter().position(|c| c == &b':').unwrap() + 1..]
//                 .split(|c| c == &b',')
//                 .map(|it| if it[0] == b' ' { &it[1..] } else { it })
//                 .map(|it| atoi::atoi(it).unwrap())
//                 .collect();
//         }
//
//         if &line[..4] == b"  Op" {
//             let mut op_and_num = line[24..].split(|c| c == &b' ');
//             let op = op_and_num.next().unwrap();
//             let num = op_and_num.next().unwrap();
//             let fun: (usize, fn(usize, usize) -> usize) = match (op[0], num) {
//                 (b'*', b"old") => (1, |nn, old| nn * old * old),
//                 (b'*', n) => {
//                     let no = atoi::atoi(n).unwrap();
//                     (no, |nn, old| nn * old)
//                 }
//                 (b'+', n) => {
//                     let no = atoi::atoi(n).unwrap();
//                     (no, |nn, old| nn + old)
//                 }
//                 _ => {
//                     unreachable!();
//                 }
//             };
//             operation = fun;
//         }
//
//         if &line[..4] == b"Te" {
//             divisible = atoi::atoi(&line[21..]).unwrap();
//         }
//
//         if line[7] == b't' {
//             condition = atoi::atoi(&line[30..]).unwrap();
//         }
//
//         if line[7] == b'f' {
//             conditions = (condition, atoi::atoi(&line[30..]).unwrap());
//         }
//
//         monkeys.push(Monkey {
//             stack,
//             op: operation,
//             div: divisible,
//             conditions,
//         });
//     }
//
//     monkeys
// }
//
