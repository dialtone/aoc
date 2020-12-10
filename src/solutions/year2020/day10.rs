// day10 parse             time:   [2.1756 us 2.1950 us 2.2159 us]
// day10 part 1            time:   [1.2428 us 1.2577 us 1.2739 us]
// day10 part 2            time:   [1.9422 us 1.9579 us 1.9796 us]

use super::*;

use num_integer::binomial;

type Input = String;
type Parsed = Vec<usize>;

pub fn part1(oinput: &Parsed) -> usize {
    let mut input = oinput.clone();
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort();
    let mut diff3 = 0;
    let mut diff1 = 0;
    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];
        if diff == 3 {
            diff3 += 1;
        } else if diff == 1 {
            diff1 += 1;
        } else {
            unreachable!()
        }
    }
    diff3 * diff1
}

pub fn part2(oinput: &Parsed) -> usize {
    let mut input = oinput.clone();
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort();
    let mut diff1_sequence = 0;
    let mut replaceable = 1;
    for i in 1..input.len() {
        let diff = input[i] - input[i - 1];
        if diff == 3 {
            if diff1_sequence > 2 {
                let removable = diff1_sequence - 2;
                let mut prod = binomial(removable, 0) + binomial(removable, 1);
                if removable >= 2 {
                    prod += binomial(removable, 2);
                }
                replaceable *= prod;
            }
            diff1_sequence = 0;
        } else if diff == 1 {
            if diff1_sequence == 0 {
                // first one is always part of the sequence
                diff1_sequence += 1;
            }
            diff1_sequence += 1;
        } else {
            unreachable!()
        }
    }
    replaceable
}

pub fn parse(s: &Input) -> Parsed {
    s.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10() {
        let test_input = "16
10
15
5
1
11
7
19
6
12
4"
        .to_owned();
        assert_eq!(part1(&parse(&test_input)), 35);
        assert_eq!(part2(&parse(&test_input)), 8);

        let test_input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        .to_owned();
        assert_eq!(part2(&parse(&test_input)), 19208);
    }

    #[test]
    fn day10() {
        let input = get_input(2020, 10).unwrap();
        assert_eq!(part1(&parse(&input)), 2277);
        assert_eq!(part2(&parse(&input)), 37024595836928);
    }
}
