use super::*;
use std::collections::*;
use std::ops::RangeInclusive;

type Input = String;
type Parsed = HashMap<(isize, isize, isize), char>;
type Parsed4 = HashMap<(isize, isize, isize, isize), char>;

pub fn part1(input: &Parsed) -> usize {
    let mut last = input.clone();
    for cycle in 0..6 {
        let mut next = HashMap::new();
        let (range_x, range_y, range_z) = get_ranges(&last);
        for z in range_z.clone() {
            for y in range_y.clone() {
                for x in range_x.clone() {
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
        last = next.clone();
    }
    last.iter().filter(|(_, v)| **v == '#').count()
}

fn printmap(map: &Parsed) {
    let (xs, ys, zs) = get_ranges(map);
    for z in zs {
        println!("Z={}", z);
        for y in ys.clone() {
            let mut v = Vec::new();
            for x in xs.clone() {
                v.push(map.get(&(x, y, z)).unwrap_or(&'.').to_string());
            }
            println!("{}", v.join(""));
        }
    }
}

fn get_ranges(
    map: &Parsed,
) -> (
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
) {
    let mut min_x = 100000000000;
    let mut max_x = -100000000000;
    let mut min_y = 1000000000000;
    let mut max_y = -10000000000000;
    let mut min_z = 10000000000000;
    let mut max_z = -10000000000000;
    for (k, _) in map.iter() {
        if k.0 < min_x {
            min_x = k.0;
        }
        if k.0 > max_x {
            max_x = k.0;
        }
        if k.1 < min_y {
            min_y = k.1;
        }
        if k.1 > max_y {
            max_y = k.1;
        }
        if k.2 < min_z {
            min_z = k.2;
        }
        if k.2 > max_z {
            max_z = k.2;
        }
    }
    (
        min_x - 1..=max_x + 1,
        min_y - 1..=max_y + 1,
        min_z - 1..=max_z + 1,
    )
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

fn get_ranges4(
    map: &Parsed4,
) -> (
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
) {
    let mut min_x = 100000000000;
    let mut max_x = -100000000000;
    let mut min_y = 1000000000000;
    let mut max_y = -10000000000000;
    let mut min_z = 10000000000000;
    let mut max_z = -10000000000000;
    let mut min_w = 10000000000000;
    let mut max_w = -10000000000000;

    for (k, _) in map.iter() {
        if k.0 < min_x {
            min_x = k.0;
        }
        if k.0 > max_x {
            max_x = k.0;
        }
        if k.1 < min_y {
            min_y = k.1;
        }
        if k.1 > max_y {
            max_y = k.1;
        }
        if k.2 < min_z {
            min_z = k.2;
        }
        if k.2 > max_z {
            max_z = k.2;
        }
        if k.3 < min_w {
            min_w = k.3;
        }
        if k.3 > max_w {
            max_w = k.3;
        }
    }
    (
        min_x - 1..=max_x + 1,
        min_y - 1..=max_y + 1,
        min_z - 1..=max_z + 1,
        min_w - 1..=max_w + 1,
    )
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

pub fn part2(input: &Parsed4) -> usize {
    let mut last = input.clone();
    for cycle in 0..6 {
        let mut next = HashMap::new();
        let (range_x, range_y, range_z, range_w) = get_ranges4(&last);
        for w in range_w.clone() {
            for z in range_z.clone() {
                for y in range_y.clone() {
                    for x in range_x.clone() {
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
        last = next.clone();
    }
    last.iter().filter(|(_, v)| **v == '#').count()
}

pub fn parse(s: &Input) -> Parsed {
    let mut map = HashMap::new();
    for (row, l) in s.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            map.insert((col as isize, row as isize, 0 as isize), c);
        }
    }
    map
}

pub fn parse4(s: &Input) -> Parsed4 {
    let mut map = HashMap::new();
    for (row, l) in s.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            map.insert((col as isize, row as isize, 0 as isize, 0 as isize), c);
        }
    }
    map
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
###"
        .to_owned();
        assert_eq!(part1(&parse(&test_input)), 112);
        assert_eq!(part2(&parse4(&test_input)), 848);
    }

    #[test]
    fn day17() {
        let input = get_input(2020, 17).unwrap();
        assert_eq!(part1(&parse(&input)), 324);
        assert_eq!(part2(&parse4(&input)), 1836);
    }
}
