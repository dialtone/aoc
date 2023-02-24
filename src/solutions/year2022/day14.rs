use itertools::Itertools;

const W: usize = 800;
const H: usize = 175;

type Cave = [[bool; W]; H];

pub fn parse(input: &str) -> (Cave, usize) {
    let mut cave: Cave = [[false; W]; H];

    let mut lowest = 0;

    for coords in input.lines().filter(|l| !l.is_empty()).map(|l| {
        l.split(" -> ").map(|coord| {
            coord
                .split_once(',')
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .unwrap()
        })
    }) {
        for (from, to) in coords.tuple_windows() {
            lowest = lowest.max(from.1.max(to.1));

            match (from, to) {
                ((fromx, fromy), (tox, toy)) if fromx == tox => {
                    for y in fromy.min(toy)..=fromy.max(toy) {
                        cave[y][fromx] = true;
                    } // horizontal line
                }
                ((fromx, fromy), (tox, toy)) if fromy == toy => {
                    for x in fromx.min(tox)..=fromx.max(tox) {
                        cave[fromy][x] = true;
                    } // Vertical line
                }
                _ => unreachable!(),
            }
        }
    }

    (cave, lowest)
}

// year 22 day14 part 1    time:   [167.78 µs 167.95 µs 168.15 µs]
pub fn part1(input: &str) -> usize {
    let (mut cave, maxy) = parse(input);
    let mut sand = 0;

    let start = (500, 0);
    let dirs = [(0, 1), (-1, 1), (1, 1)];

    let mut pos = start;
    while pos.1 <= maxy {
        let mut stable = true;
        for d in dirs {
            let newpos = ((pos.0 as isize + d.0) as usize, pos.1 + d.1);
            if cave[newpos.1][newpos.0] {
                continue;
            }
            pos = newpos;
            stable = false;
            break;
        }
        if stable {
            cave[pos.1][pos.0] = true;
            pos = start;
            sand += 1;
        }
    }

    sand
}

// year 22 day14 part 2    time:   [4.5934 ms 4.6069 ms 4.6231 ms]
pub fn part2(input: &str) -> usize {
    let (mut cave, mut maxy) = parse(input);
    let mut sand = 0;
    maxy += 2;

    let start = (500, 0);
    let dirs = [(0, 1), (-1, 1), (1, 1)];

    let mut pos = start;
    loop {
        let mut stable = true;
        for d in dirs {
            let newpos = ((pos.0 as isize + d.0) as usize, pos.1 + d.1);
            if newpos.1 == maxy || cave[newpos.1][newpos.0] {
                continue;
            }
            pos = newpos;
            stable = false;
            break;
        }
        if stable {
            sand += 1;
            if pos == start {
                break;
            }
            cave[pos.1][pos.0] = true;
            pos = start;
        }
    }

    sand
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
