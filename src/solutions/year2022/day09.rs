use rustc_hash::FxHashSet;

enum Move {
    R(u8),
    L(u8),
    D(u8),
    U(u8),
}

impl Move {
    pub fn from_str(input: &str) -> Move {
        match input.split_once(' ') {
            Some(("R", n)) => Move::R(n.parse::<u8>().unwrap()),
            Some(("L", n)) => Move::L(n.parse::<u8>().unwrap()),
            Some(("U", n)) => Move::U(n.parse::<u8>().unwrap()),
            Some(("D", n)) => Move::D(n.parse::<u8>().unwrap()),
            _ => unreachable!(),
        }
    }
}

pub fn distance(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    (h.0 - t.0, h.1 - t.1)
}

pub fn mov_t(h: (i32, i32), mut t: (i32, i32)) -> (i32, i32) {
    match distance(h, t) {
        (x, y) if x.abs() <= 1 && y.abs() <= 1 => {}
        (0, y) => {
            t = (t.0, t.1 + (y / 2));
        }
        (x, 0) => {
            t = (t.0 + (x / 2), t.1);
        }
        (x, y) if y == 1 || y == -1 => {
            t = (t.0 + (x / 2), t.1 + y);
        }
        (x, y) if x == 1 || x == -1 => {
            t = (t.0 + x, t.1 + (y / 2));
        }
        _ => unreachable!(), // we should be called for each loop
    }
    t
}

pub fn part1(input: &str) -> usize {
    let moves: Vec<Move> = input.lines().map(Move::from_str).collect();
    let mut seen: FxHashSet<(i32, i32)> = FxHashSet::default();

    let start = (0, 0);
    let mut h = start;
    let mut t = start;

    for m in moves {
        match m {
            Move::R(n) => {
                for _ in 0..n {
                    h = (h.0 + 1, h.1);
                    t = mov_t(h, t);
                    seen.insert(t);
                }
            }
            Move::L(n) => {
                for _ in 0..n {
                    h = (h.0 - 1, h.1);
                    t = mov_t(h, t);
                    seen.insert(t);
                }
            }
            Move::D(n) => {
                for _ in 0..n {
                    h = (h.0, h.1 - 1);
                    t = mov_t(h, t);
                    seen.insert(t);
                }
            }
            Move::U(n) => {
                for _ in 0..n {
                    h = (h.0, h.1 + 1);
                    t = mov_t(h, t);
                    seen.insert(t);
                }
            }
        }
    }
    seen.len()
}

pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day09_test() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part1(input), 13);
        // assert_eq!(part2(input), 8);
    }

    #[test]
    fn day09() {
        let input = get_input(2022, 9).unwrap();
        assert_eq!(part1(&input), 6337);
        // assert_eq!(part2(&input), 291840);
    }
}
