use super::*;

type Input = String;
type Time = usize;
type Parsed = Vec<usize>;

pub fn part1(goal: Time, input: &Parsed) -> usize {
    let mut res = Vec::new();
    for num in input {
        if *num == 0 {
            continue;
        }
        let div = num_integer::div_ceil(goal, *num);
        res.push((div * num - goal, *num));
    }
    res.sort_by_key(|&(a, _)| a);
    res[0].0 * res[0].1
}

pub fn part2(input: Parsed) -> usize {
    let mut last = 1;
    let mut prod = 1;
    for (i, &num) in input.iter().enumerate() {
        if num == 0 {
            continue;
        }
        // since all numbers are prime and co-prime you can
        // just multiply them together to search for bigger
        // numbers, there are no shared multiplers that you
        // need to keep in mind.

        // So the basic search algorithm is that you are going to
        // search first for the first 2 numbers until you find the
        // first multiple of the cumulative product of numbers so far
        // that has a remainder equal to the remainder you are looking for
        // that means that if your number is N and its sequence is i
        // you are looking for a place where
        // N * prev % 13 == 13 - 1
        // of course you'll need to deal with the - 1 (i) being bigger than num in
        // some cases so you'll need to crop it down to the number
        for x in (last..).step_by(prod) {
            if x % num == (num - (i % num)) % num {
                last = x;
                prod *= num;
                break;
            }
        }
    }
    last
}

pub fn parse(s: &Input) -> (Time, Parsed) {
    let mut l = s.lines();
    let time = l.next().unwrap().parse().unwrap();

    (
        time,
        l.next()
            .unwrap()
            .split(",")
            .map(|i| i.parse().unwrap_or(0))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13() {
        let input = "939
7,13,x,x,59,x,31,19"
            .to_owned();
        let (goal, buses) = parse(&input);
        // assert_eq!(part1(goal, &buses), 295);
        assert_eq!(part2(buses), 1068781);
    }

    #[test]
    fn day13() {
        let input = get_input(2020, 13).unwrap();
        let (goal, buses) = parse(&input);
        assert_eq!(part1(goal, &buses), 333);
        assert_eq!(part2(buses), 690123192779524);
    }
}
