use itertools::Itertools;
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
        (x, y) if y.abs() == 1 => {
            t = (t.0 + (x / 2), t.1 + y);
        }
        (x, y) if x.abs() == 1 => {
            t = (t.0 + x, t.1 + (y / 2));
        }
        (x, y) if x.abs() == 2 => {
            t = (t.0 + (x / 2), t.1 + (y / 2));
        }
        _ => {
            println!("h:{:?} t:{:?} d:{:?}", h, t, distance(h, t));
            unreachable!();
        } // we should be called for each loop
    }
    t
}

// year 22 day09 part 1    time:   [146.93 µs 147.83 µs 148.72 µs]
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

// year 22 day09 part 2    time:   [369.15 µs 370.10 µs 371.07 µs]
pub fn part2(input: &str) -> usize {
    let mut snake: Vec<(i32, i32)> = vec![(0, 0); 10];
    let moves: Vec<Move> = input.lines().map(Move::from_str).collect();
    let mut seen: FxHashSet<(i32, i32)> = FxHashSet::default();

    for m in moves {
        match m {
            Move::R(n) => {
                for _ in 0..n {
                    let h = (snake[0].0 + 1, snake[0].1);
                    snake[0] = h;
                    for (i, (j, k)) in (0..10).tuple_windows().enumerate() {
                        snake[i + 1] = mov_t(snake[j], snake[k]);
                    }
                    seen.insert(*snake.last().unwrap());
                }
            }
            Move::L(n) => {
                for _ in 0..n {
                    let h = (snake[0].0 - 1, snake[0].1);
                    snake[0] = h;
                    for (i, (j, k)) in (0..10).tuple_windows().enumerate() {
                        snake[i + 1] = mov_t(snake[j], snake[k]);
                    }
                    seen.insert(*snake.last().unwrap());
                }
            }
            Move::D(n) => {
                for _ in 0..n {
                    let h = (snake[0].0, snake[0].1 - 1);
                    snake[0] = h;
                    for (i, (j, k)) in (0..10).tuple_windows().enumerate() {
                        snake[i + 1] = mov_t(snake[j], snake[k]);
                    }
                    seen.insert(*snake.last().unwrap());
                }
            }
            Move::U(n) => {
                for _ in 0..n {
                    let h = (snake[0].0, snake[0].1 + 1);
                    snake[0] = h;
                    for (i, (j, k)) in (0..10).tuple_windows().enumerate() {
                        snake[i + 1] = mov_t(snake[j], snake[k]);
                    }
                    seen.insert(*snake.last().unwrap());
                }
            }
        }
    }
    seen.len()
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
        assert_eq!(part2(input), 1);
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(input), 36);
    }

    #[test]
    fn day09() {
        let input = get_input(2022, 9).unwrap();
        assert_eq!(part1(&input), 6337);
        assert_eq!(part2(&input), 2455);
    }
}
