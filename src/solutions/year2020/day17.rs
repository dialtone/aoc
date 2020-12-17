// day17 part 1            time:   [16.159 ms 16.321 ms 16.500 ms]
// day17 part 2            time:   [487.51 ms 490.13 ms 492.92 ms]
// https://github.com/smmalis37/aoc2020/blob/main/src/days/day17.rs
// this ^ is a much more efficient way of doing it.
use super::*;
use std::collections::*;

pub fn part1(input: &str) -> usize {
    let mut min_x: isize = 0;
    let mut max_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_y: isize = 0;
    let mut min_z: isize = 0;
    let mut max_z: isize = 0;

    let mut map = HashMap::new();
    for (row, l) in input.lines().enumerate() {
        if row > max_y as usize {
            max_y = row as isize;
        }
        for (col, c) in l.chars().enumerate() {
            if col > max_x as usize {
                max_x = col as isize;
            }
            map.insert((col as isize, row as isize, 0 as isize), c);
        }
    }

    let mut last = map;
    for _cycle in 0..6 {
        min_x -= 1;
        max_x += 1;
        min_y -= 1;
        max_y += 1;
        min_z -= 1;
        max_z += 1;

        let mut next = HashMap::new();
        for z in min_z..=max_z {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let k = (x, y, z);
                    let v = last.get(&k).unwrap_or(&'.');
                    if *v == '#' {
                        let actives = neighbors(&k)
                            .iter()
                            .map(|coords| last.get(coords).unwrap_or(&'.'))
                            .filter(|&&v| v == '#')
                            .count();
                        if actives == 2 || actives == 3 {
                            next.insert(k, '#');
                        } else {
                            next.insert(k, '.');
                        }
                    } else {
                        if neighbors(&k)
                            .iter()
                            .map(|coords| last.get(coords).unwrap_or(&'.'))
                            .filter(|&&v| v == '#')
                            .count()
                            == 3
                        {
                            next.insert(k, '#');
                        } else {
                            next.insert(k, '.');
                        }
                    }
                }
            }
        }
        last = next;
    }
    last.iter().filter(|(_, v)| **v == '#').count()
}

fn neighbors(coords: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    let mut res = Vec::new();
    for x in coords.0 - 1..=coords.0 + 1 {
        for y in coords.1 - 1..=coords.1 + 1 {
            for z in coords.2 - 1..=coords.2 + 1 {
                if coords.0 == x && coords.1 == y && coords.2 == z {
                    continue;
                }
                res.push((x, y, z));
            }
        }
    }
    res
}

fn neighbors4(coords: &(isize, isize, isize, isize)) -> Vec<(isize, isize, isize, isize)> {
    let mut res = Vec::new();
    for x in coords.0 - 1..=coords.0 + 1 {
        for y in coords.1 - 1..=coords.1 + 1 {
            for z in coords.2 - 1..=coords.2 + 1 {
                for w in coords.3 - 1..=coords.3 + 1 {
                    if coords.0 == x && coords.1 == y && coords.2 == z && coords.3 == w {
                        continue;
                    }
                    res.push((x, y, z, w));
                }
            }
        }
    }
    res
}

pub fn part2(s: &str) -> usize {
    let mut min_x: isize = 0;
    let mut max_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_y: isize = 0;
    let mut min_z: isize = 0;
    let mut max_z: isize = 0;
    let mut min_w: isize = 0;
    let mut max_w: isize = 0;

    let mut map = HashMap::new();
    for (row, l) in s.lines().enumerate() {
        if row > max_y as usize {
            max_y = row as isize;
        }
        for (col, c) in l.chars().enumerate() {
            if col > max_x as usize {
                max_x = col as isize;
            }
            map.insert((col as isize, row as isize, 0 as isize, 0 as isize), c);
        }
    }

    let mut last = map;
    for _cycle in 0..6 {
        min_x -= 1;
        max_x += 1;
        min_y -= 1;
        max_y += 1;
        min_z -= 1;
        max_z += 1;
        min_w -= 1;
        max_w += 1;

        let mut next = HashMap::new();
        for w in min_w..=max_w {
            for z in min_z..=max_z {
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        let k = (x, y, z, w);
                        let v = last.get(&k).unwrap_or(&'.');
                        if *v == '#' {
                            let actives = neighbors4(&k)
                                .iter()
                                .map(|coords| last.get(coords).unwrap_or(&'.'))
                                .filter(|&&v| v == '#')
                                .count();
                            if actives == 2 || actives == 3 {
                                next.insert(k, '#');
                            } else {
                                next.insert(k, '.');
                            }
                        } else {
                            if neighbors4(&k)
                                .iter()
                                .map(|coords| last.get(coords).unwrap_or(&'.'))
                                .filter(|&&v| v == '#')
                                .count()
                                == 3
                            {
                                next.insert(k, '#');
                            } else {
                                next.insert(k, '.');
                            }
                        }
                    }
                }
            }
        }
        last = next;
    }
    last.iter().filter(|(_, v)| **v == '#').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_test() {
        assert_eq!(neighbors(&(0, 0, 0)).len(), 26);
    }

    #[test]
    fn test_day17() {
        let test_input = ".#.
..#
###";
        assert_eq!(part1(&test_input), 112);
        assert_eq!(part2(&test_input), 848);
    }

    #[test]
    fn day17() {
        let input = get_input(2020, 17).unwrap();
        assert_eq!(part1(&input), 324);
        assert_eq!(part2(&input), 1836);
    }
}
