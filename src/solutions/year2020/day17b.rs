// day17b part 1           time:   [787.51 us 793.55 us 800.59 us]
// day17b part 2           time:   [14.821 ms 15.026 ms 15.257 ms]
use super::*;
use std::collections::*;
use std::convert::TryInto;

macro_rules! neighbors {
    ($xrange:expr; $yrange:expr; $zrange:expr
    ) => {{
        let mut _a: Vec<_> = Vec::new();
        for _x in $xrange {
            for _y in $yrange {
                for _z in $zrange {
                    if _x == 0 && _y == 0 && _z == 0 {
                        continue;
                    }
                    _a.push((_x, _y, _z));
                }
            }
        }
        _a.try_into().unwrap()
    }};
    ($xrange:expr; $yrange:expr; $zrange:expr; $wrange:expr
    ) => {{
        let mut _a: Vec<_> = Vec::new();
        for _x in $xrange {
            for _y in $yrange {
                for _z in $zrange {
                    for _w in $wrange {
                        if _x == 0 && _y == 0 && _z == 0 && _w == 0 {
                            continue;
                        }
                        _a.push((_x, _y, _z, _w));
                    }
                }
            }
        }
        _a.try_into().unwrap()
    }};
}

lazy_static! {
    static ref NEIGHBORS3: [(isize, isize, isize); 26] = neighbors![-1..2; -1..2; -1..2];
    static ref NEIGHBORS4: [(isize, isize, isize, isize); 80] =
        neighbors![-1..2; -1..2; -1..2; -1..2];
    // static ref NEIGH3: [(isize, isize, isize); 26] = iproduct!(-1..2, -1..2, -1..2)
    //     .collect::<Vec<_>>()
    //     .try_into()
    //     .unwrap();
}

pub fn part1(input: &str) -> usize {
    let mut map = HashSet::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                map.insert((x as isize, y as isize, 0 as isize));
            }
        }
    }

    let mut adj: HashMap<(isize, isize, isize), usize> = HashMap::new();
    for _cycle in 0..6 {
        adj.clear();
        for &cube in &map {
            for &ds in NEIGHBORS3.iter() {
                *adj.entry((cube.0 + ds.0, cube.1 + ds.1, cube.2 + ds.2))
                    .or_default() += 1;
            }
        }

        let mut new = Vec::new();

        for cnt in &adj {
            let (&cd, &cnt) = cnt;
            if !map.contains(&cd) {
                if cnt == 3 {
                    new.push(cd);
                }
            } else if cnt == 2 || cnt == 3 {
                new.push(cd);
            }
        }
        map.clear();
        map.extend(new);

        // another approach
        // let mut new = Vec::new();
        // for &pos in &map {
        //     let cnt = *adj.get(&pos).unwrap_or(&0);
        //     if cnt == 3 || cnt == 2 {
        //         new.push(pos);
        //     }
        // }

        // for (&pos, &cnt) in &adj {
        //     if !map.contains(&pos) && cnt == 3 {
        //         new.push(pos);
        //     }
        // }

        // map.clear();
        // map.extend(new);
    }

    map.len()
}

pub fn part2(input: &str) -> usize {
    let mut map = HashSet::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                map.insert((x as isize, y as isize, 0 as isize, 0 as isize));
            }
        }
    }

    let mut adj: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
    for _cycle in 0..6 {
        adj.clear();
        for &cube in &map {
            for &ds in NEIGHBORS4.iter() {
                *adj.entry((cube.0 + ds.0, cube.1 + ds.1, cube.2 + ds.2, cube.3 + ds.3))
                    .or_default() += 1;
            }
        }

        let mut new = Vec::new();

        for cnt in &adj {
            let (&cd, &cnt) = cnt;
            if !map.contains(&cd) {
                if cnt == 3 {
                    new.push(cd);
                }
            } else if cnt == 2 || cnt == 3 {
                new.push(cd);
            }
        }
        map.clear();
        map.extend(new);
    }

    map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17b() {
        let test_input = ".#.
..#
###";
        assert_eq!(part1(&test_input), 112);
        assert_eq!(part2(&test_input), 848);
    }

    #[test]
    fn day17b() {
        let input = get_input(2020, 17).unwrap();
        assert_eq!(part1(&input), 324);
        assert_eq!(part2(&input), 1836);
    }
}
