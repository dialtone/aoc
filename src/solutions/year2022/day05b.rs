type Move = (usize, usize, usize);
type Stacks = Vec<Vec<char>>;
type ParsedData = (Stacks, Vec<Move>);

pub fn part1(input: &str) -> String {
    let (mut stacks, moves) = parse(input);
    for (cnt, from, to) in moves {
        for _ in 0..cnt {
            let moved = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(moved);
        }
    }
    stacks.iter().filter_map(|s| s.last()).collect()
}

pub fn part2(input: &str) -> String {
    let (mut stacks, moves) = parse(input);
    for (cnt, from, to) in moves {
        let from_stack = &mut stacks[from - 1];
        let mut suffix = from_stack.drain((from_stack.len() - cnt)..).collect();
        stacks[to - 1].append(&mut suffix);
    }
    stacks.iter().filter_map(|s| s.last()).collect()
}

fn parse(input: &str) -> ParsedData {
    let mut moves = vec![];
    let mut crates = vec![vec!(); 9];
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
                slot.push(c);
            } else {
                break;
            }
        }
    }

    for slot in crates.iter_mut() {
        slot.reverse();
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
