// use crate::solutions::clear_screen;
// use crate::solutions::go_to_top;

#[derive(Debug)]
pub struct Monkey {
    pub stack: Vec<usize>,
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
            .collect::<Vec<usize>>();

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

// year 22 day11 part 1    time:   [4.9819 µs 4.9915 µs 5.0028 µs]
pub fn part1(input: &[u8]) -> usize {
    let mut monkeys = parse(input);
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let len = monkeys[i].stack.len();
            for j in 0..len {
                let item = monkeys[i].stack[j];
                inspections[i] += 1;
                let new_item = monkeys[i].op.1(monkeys[i].op.0, item) / 3;
                let throw_to = if new_item % monkeys[i].div == 0 {
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

// year 22 day11 part 2    time:   [2.0104 ms 2.0120 ms 2.0138 ms]
pub fn part2(input: &[u8]) -> usize {
    let mut monkeys = parse(input);
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];

    let mcm = monkeys.iter().map(|m| m.div).product::<usize>();
    // clear_screen();
    // go_to_top();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let len = monkeys[i].stack.len();
            for j in 0..len {
                let item = monkeys[i].stack[j];
                inspections[i] += 1;
                // go_to_top();
                // println!("{:?}", inspections);
                let new_item = monkeys[i].op.1(monkeys[i].op.0, item) % mcm;
                let throw_to = if new_item % monkeys[i].div == 0 {
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
