use itertools::Itertools;
use rustc_hash::FxHashSet;

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
        // diagonal, only possible when reacting to other non-head
        (x, y) if x.abs() == 2 && y.abs() == 2 => {
            t = (t.0 + (x / 2), t.1 + (y / 2));
        }
        _ => {
            // we should be called for each loop, so this doesn't happen
            println!("h:{:?} t:{:?} d:{:?}", h, t, distance(h, t));
            unreachable!();
        }
    }
    t
}

// year 22 day09 part 1    time:   [127.63 µs 128.48 µs 129.45 µs]
pub fn part1(input: &[u8]) -> usize {
    let moves = input
        .split(|c| c == &b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| match (l[0], atoi::atoi(&l[2..]).unwrap()) {
            (b'R', n) => ((1, 0), n),
            (b'L', n) => ((-1, 0), n),
            (b'U', n) => ((0, 1), n),
            (_, n) => ((0, -1), n),
        });
    let mut seen: FxHashSet<(i32, i32)> = FxHashSet::default();

    let start = (0, 0);
    let mut h = start;
    let mut t = start;

    for (d, n) in moves {
        for _ in 0..n {
            h = (h.0 + d.0, h.1 + d.1);
            t = mov_t(h, t);
            seen.insert(t);
        }
    }
    seen.len()
}

// year 22 day09 part 2    time:   [238.48 µs 239.17 µs 239.92 µs]
pub fn part2(input: &[u8]) -> usize {
    let mut snake: Vec<(i32, i32)> = vec![(0, 0); 10];
    let moves = input
        .split(|c| c == &b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| match (l[0], atoi::atoi(&l[2..]).unwrap()) {
            (b'R', n) => ((1, 0), n),
            (b'L', n) => ((-1, 0), n),
            (b'U', n) => ((0, 1), n),
            (_, n) => ((0, -1), n),
        });

    // let moves: Vec<Move> = input.lines().map(Move::from_str).collect();
    let mut seen: FxHashSet<(i32, i32)> = FxHashSet::default();

    for (d, n) in moves {
        for _ in 0..n {
            snake[0] = (snake[0].0 + d.0, snake[0].1 + d.1);
            for (i, (j, k)) in (0..10).tuple_windows().enumerate() {
                snake[i + 1] = mov_t(snake[j], snake[k]);
            }
            seen.insert(*snake.last().unwrap());
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
        assert_eq!(part1(input.as_bytes()), 13);
        assert_eq!(part2(input.as_bytes()), 1);
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(input.as_bytes()), 36);
    }

    #[test]
    fn day09() {
        let input = get_input(2022, 9).unwrap();
        assert_eq!(part1(input.as_bytes()), 6337);
        assert_eq!(part2(input.as_bytes()), 2455);
    }
}
