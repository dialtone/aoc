use super::*;

use std::collections::HashMap;

type Input = String;
type Parsed = Vec<usize>;

pub fn part1(input: &Parsed) -> usize {
    solve(input, 2020)
}

fn solve(input: &Parsed, upto: usize) -> usize {
    let mut occurrences: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut start = 0;
    let mut curr = 0;
    loop {
        if start == upto {
            return curr;
        }

        if start < input.len() {
            curr = input[start];
        } else {
            curr = if let Some(v) = occurrences.get(&curr) {
                if v.len() == 1 {
                    0
                } else {
                    let last = v[v.len() - 1];
                    let second_to_last = v[v.len() - 2];
                    last - second_to_last
                }
            } else {
                unreachable!()
            }
        }

        occurrences
            .entry(curr)
            .or_insert(Vec::new())
            .push(start + 1);

        // dbg!(curr, start + 1);

        start += 1;
    }
}

pub fn part2(input: &Parsed) -> usize {
    solve(input, 30000000)
}

pub fn parse(s: &Input) -> Parsed {
    s.split(",").map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15() {
        let test_input = "0,3,6".to_owned();
        assert_eq!(part1(&parse(&test_input)), 436);
        assert_eq!(part2(&parse(&test_input)), 175594);
    }

    #[test]
    fn day15() {
        let input = "0,5,4,1,10,14,7".to_owned(); // get_input(2020, 15).unwrap();
        assert_eq!(part1(&parse(&input)), 203);
        assert_eq!(part2(&parse(&input)), 9007186);
    }
}
