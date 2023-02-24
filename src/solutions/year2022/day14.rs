use itertools::Itertools;
use num::range_step_inclusive;
use rustc_hash::FxHashSet;

static GROUND: char = '#';
static AIR: char = '.';
static SAND: char = 'o';

pub fn printmap(cave: &FxHashSet<(i16, i16)>) {
    let minx = cave.iter().map(|(x, _)| *x).min().unwrap();
    let maxx = cave.iter().map(|(x, _)| *x).max().unwrap();
    let miny = 0;
    let maxy = cave.iter().map(|(_, y)| *y).max().unwrap();

    println!();
    for y in miny..=maxy {
        for x in minx..=maxx {
            if cave.contains(&(x, y)) {
                print!("{}", GROUND);
            } else {
                print!("{}", AIR);
            }
        }
        println!();
    }
}

pub fn parse(input: &str) -> FxHashSet<(i16, i16)> {
    let mut cave: FxHashSet<(i16, i16)> = FxHashSet::default();

    for coords in input.lines().filter(|l| !l.is_empty()).map(|l| {
        l.split(" -> ").map(|coord| {
            coord
                .split_once(',')
                .map(|(x, y)| (x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()))
                .unwrap()
        })
    }) {
        for (from, to) in coords.tuple_windows() {
            match (from, to) {
                ((fromx, fromy), (tox, toy)) if fromx == tox => {
                    for y in range_step_inclusive(fromy, toy, if fromy > toy { -1 } else { 1 }) {
                        cave.insert((fromx, y));
                    } // horizontal line
                }
                ((fromx, fromy), (tox, toy)) if fromy == toy => {
                    for x in range_step_inclusive(fromx, tox, if fromx > tox { -1 } else { 1 }) {
                        cave.insert((x, fromy));
                    } // Vertical line
                }
                _ => unreachable!(),
            }
        }
    }

    cave
}

// year 22 day14 part 1    time:   [377.12 µs 377.71 µs 378.39 µs]
pub fn part1(input: &str) -> usize {
    let mut cave = parse(input);
    // if a grain of sands goes beyond this y, we should stop computation as nothing else will
    // stabilize
    let maxy = cave.iter().map(|(_, y)| *y).max().unwrap();

    // printmap(&cave);

    let start = (500, 0);
    let dirs = [(0, 1), (-1, 1), (1, 1)];

    let walls = cave.len();
    let mut pos = start;
    while pos.1 <= maxy {
        let mut stable = true;
        for d in dirs {
            let newpos = (pos.0 + d.0, pos.1 + d.1);
            if cave.contains(&newpos) {
                continue;
            }
            pos = newpos;
            stable = false;
            break;
        }
        if stable {
            cave.insert(pos);
            pos = start;
        }
    }

    // printmap(&cave);

    cave.len() - walls
}

// year 22 day14 part 2    time:   [16.963 ms 16.994 ms 17.028 ms]
pub fn part2(input: &str) -> usize {
    let mut cave = parse(input);

    let maxy = cave.iter().map(|(_, y)| *y).max().unwrap() + 2;

    // printmap(&cave);

    let start = (500, 0);
    let dirs = [(0, 1), (-1, 1), (1, 1)];

    let walls = cave.len();
    let mut pos = start;
    loop {
        let mut stable = true;
        for d in dirs {
            let newpos = (pos.0 + d.0, pos.1 + d.1);
            if cave.contains(&newpos) || newpos.1 == maxy {
                continue;
            }
            pos = newpos;
            stable = false;
            break;
        }
        if stable {
            if pos == start {
                break;
            }
            cave.insert(pos);
            pos = start;
        }
    }

    // printmap(&cave);

    cave.len() - walls + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn example() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part1(input), 24);
        assert_eq!(part2(input), 93);
    }

    #[test]
    fn problem() {
        let input = get_input(2022, 14).unwrap();
        assert_eq!(part1(&input), 843);
        assert_eq!(part2(&input), 27625);
    }
}
