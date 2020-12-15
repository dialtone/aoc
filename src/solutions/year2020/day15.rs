// day15 parse             time:   [391.75 ns 397.76 ns 404.48 ns]
// day15 part 1            time:   [5.4187 us 5.4735 us 5.5365 us]
// day15 part 2            time:   [935.75 ms 940.06 ms 944.73 ms]

use super::*;

type Input = String;
type Parsed = Vec<usize>;

pub fn part1(input: &Parsed) -> usize {
    solve(input, 2020)
}

fn solve(input: &Parsed, upto: usize) -> usize {
    let mut occurrences: Vec<Option<usize>> = vec![None; upto];

    input.iter().copied().enumerate().for_each(|(i, n)| {
        occurrences[n] = Some(i);
    });

    let mut last = *input.last().unwrap();
    for i in input.len()..upto {
        let prev_last = last;
        if let Some(v) = occurrences[last] {
            last = i - 1 - v;
        } else {
            last = 0
        }
        occurrences[prev_last] = Some(i - 1);
    }
    last
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
