pub fn part1(input: &[u32]) -> u32 {
    return *(input.iter().max().unwrap());
}

pub fn part2(mut input: Vec<u32>) -> u32 {
    input.sort();
    input.reverse();
    input[..3].iter().sum()
}

pub fn parse(s: &str) -> Vec<u32> {
    let mut tot = 0;
    let mut res = vec![];

    for l in s.lines() {
        if l.is_empty() {
            res.push(tot);
            tot = 0;
        } else {
            tot += l.parse::<u32>().unwrap();
        }
    }
    if tot > 0 {
        res.push(tot);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day01_test() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let parsed = parse(input);
        assert_eq!(part1(&parsed), 24000);
        assert_eq!(part2(parsed), 45000);
    }

    #[test]
    fn day01() {
        let input = get_input(2022, 1).unwrap();
        let parsed = parse(&input);
        assert_eq!(part1(&parsed), 67622);
        assert_eq!(part2(parsed), 201491);
    }
}
