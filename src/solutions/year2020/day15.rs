// day15 parse             time:   [391.75 ns 397.76 ns 404.48 ns]
// day15 part 1            time:   [5.4187 us 5.4735 us 5.5365 us]
// day15 part 2            time:   [935.75 ms 940.06 ms 944.73 ms]

// switching to a Vec<u32> improved performance by a lot actually and using
// std::num::NonZeroU32 would yield even more improvement
// day15 parse             time:   [332.28 ns 339.25 ns 347.31 ns]
// day15 part 1            time:   [4.1781 us 4.2131 us 4.2503 us]
// day15 part 2            time:   [691.77 ms 693.68 ms 695.58 ms]

type Input = String;
type Parsed = Vec<u32>;

pub fn part1(input: &Parsed) -> u32 {
    solve(input, 2020)
}

fn solve(input: &Parsed, upto: u32) -> u32 {
    let mut occurrences: Vec<Option<u32>> = vec![None; upto as usize];

    input.iter().copied().enumerate().for_each(|(i, n)| {
        occurrences[n as usize] = Some(i as u32);
    });

    let mut last = *input.last().unwrap();
    for i in input.len() as u32..upto {
        let prev_last = last;
        if let Some(v) = occurrences[last as usize] {
            last = i - 1 - v;
        } else {
            last = 0
        }
        occurrences[prev_last as usize] = Some(i - 1);
    }
    last
}

pub fn part2(input: &Parsed) -> u32 {
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
