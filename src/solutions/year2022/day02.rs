#[derive(Copy, Clone)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from_char(c: &str) -> Self {
        match c {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_outcome(p: &Play, outcome: &str) -> Self {
        match outcome {
            "X" => p.to_lose(),
            "Y" => p.to_draw(),
            "Z" => p.to_win(),
            _ => unreachable!(),
        }
    }

    fn score(&self, other: &Play) -> u64 {
        match (self, other) {
            (Play::Rock, Play::Scissors) => 1 + 6,
            (Play::Paper, Play::Rock) => 2 + 6,
            (Play::Scissors, Play::Paper) => 3 + 6,

            (Play::Scissors, Play::Rock) => 3,
            (Play::Rock, Play::Paper) => 1,
            (Play::Paper, Play::Scissors) => 2,

            (Play::Scissors, _) => 3 + 3,
            (Play::Rock, _) => 1 + 3,
            (Play::Paper, _) => 2 + 3,
        }
    }

    fn to_win(&self) -> Play {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }

    fn to_draw(&self) -> Play {
        *self
    }

    fn to_lose(&self) -> Play {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }
}

pub fn part1(input: &Vec<(Play, Play)>) -> u64 {
    let mut total = 0;
    for (other, me) in input {
        total += me.score(other);
    }
    total
}

// pub fn part2(input: Vec<(Play, Play)>) -> u32 {
//     return 0;
// }

pub fn parse1(s: &str) -> Vec<(Play, Play)> {
    let mut res = vec![];
    for l in s.lines() {
        let mut plays = l.split(' ');
        res.push((
            Play::from_char(plays.next().unwrap()),
            Play::from_char(plays.next().unwrap()),
        ));
    }
    res
}

pub fn parse2(s: &str) -> Vec<(Play, Play)> {
    let mut res = vec![];
    for l in s.lines() {
        let mut plays = l.split(' ');
        let p = Play::from_char(plays.next().unwrap());
        let p2 = Play::from_outcome(&p, plays.next().unwrap());
        res.push((p, p2));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day02_test() {
        let input = "A Y
B X
C Z";
        let parsed = parse1(input);
        assert_eq!(part1(&parsed), 15);
        let parsed = parse2(input);
        assert_eq!(part1(&parsed), 12);
    }

    #[test]
    fn day02() {
        let input = get_input(2022, 2).unwrap();
        let parsed = parse1(&input);
        assert_eq!(part1(&parsed), 11603);
        let parsed = parse2(&input);
        assert_eq!(part1(&parsed), 12725);
    }
}
