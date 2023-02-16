use std::collections::VecDeque;

type Moves = Vec<(usize, usize, usize)>;
type Crates = Vec<VecDeque<char>>;

pub fn part1(input: &str) -> String {
    let (mut crates, moves) = parse(input);
    for (take, from_, to_) in moves {
        for _ in 0..take {
            let crate_ = crates[from_ - 1].pop_front().unwrap();
            crates[to_ - 1].push_front(crate_);
        }
    }
    let out = &crates
        .iter()
        .filter_map(|stack| stack.get(0))
        .collect::<String>();
    out.to_string()
}

pub fn part2(input: &str) -> String {
    let (mut crates, moves) = parse(input);
    for (take, from_, to_) in moves {
        let new_stack = crates[from_ - 1].split_off(take);
        while !crates[from_ - 1].is_empty() {
            if let Some(item) = crates[from_ - 1].pop_back() {
                crates[to_ - 1].push_front(item);
            }
        }
        crates[from_ - 1] = new_stack;
    }
    let out = &crates
        .iter()
        .filter_map(|stack| stack.get(0))
        .collect::<String>();
    out.to_string()
}

fn parse(input: &str) -> (Crates, Moves) {
    let mut moves = vec![];
    let mut crates = vec![VecDeque::new(); 9];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with('m') {
            let l: Vec<&str> = line.split(' ').collect();
            moves.push((
                l[1].parse::<usize>().unwrap(),
                l[3].parse::<usize>().unwrap(),
                l[5].parse::<usize>().unwrap(),
            ));
            continue;
        }

        if line.starts_with(" 1") {
            continue;
        }
        for (n, slot) in crates.iter_mut().enumerate().take(9) {
            if let Some(c) = line.chars().nth(1 + n * 4) {
                if c == ' ' {
                    continue;
                }
                slot.push_back(c);
            } else {
                break;
            }
        }
    }

    (crates, moves)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day05_test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(part1(input), "CMZ".to_string());
        assert_eq!(part2(input), "MCD".to_string());
    }

    #[test]
    fn day05() {
        let input = get_input(2022, 5).unwrap();
        assert_eq!(part1(&input), "QMBMJDFTD".to_string());
        assert_eq!(part2(&input), "NBTVTJNFJ".to_string());
    }
}
