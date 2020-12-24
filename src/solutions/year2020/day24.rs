// day24 part 1            time:   [1.3879 ms 1.4111 ms 1.4374 ms]
// day24 part 2            time:   [66.958 ms 67.954 ms 69.104 ms]

use super::*;

use std::collections::*;

type Path = Vec<String>;
type Paths = Vec<Path>;
type Pos = (isize, isize, isize);

pub fn parse(input: &str) -> Paths {
    let mut prev_b = '0';
    let mut paths: Paths = vec![];
    for line in input.lines() {
        let mut path: Path = vec![];
        for b in line.chars() {
            // e, se, sw, w, nw, and ne
            match (prev_b, b) {
                ('s' | 'n', _) => {
                    path.push([prev_b, b].iter().copied().collect());
                    prev_b = '0';
                }
                (_, 'e' | 'w') => path.push(b.to_string()),
                (_, 's' | 'n') => prev_b = b,
                _ => continue,
            }
        }
        paths.push(path);
    }
    paths
}

pub fn initialize(paths: &Paths) -> HashSet<Pos> {
    let mut black: HashSet<Pos> = HashSet::new();
    for path in paths {
        let mut pos = (0, 0, 0);
        for step in path {
            match step.as_str() {
                "ne" => pos = (pos.0, pos.1 - 1, pos.2 + 1),
                "nw" => pos = (pos.0 - 1, pos.1, pos.2 + 1),
                "e" => pos = (pos.0 + 1, pos.1 - 1, pos.2),
                "w" => pos = (pos.0 - 1, pos.1 + 1, pos.2),
                "sw" => pos = (pos.0, pos.1 + 1, pos.2 - 1),
                "se" => pos = (pos.0 + 1, pos.1, pos.2 - 1),
                _ => unreachable!(),
            }
        }
        if black.contains(&pos) {
            black.remove(&pos);
        } else {
            black.insert(pos);
        }
    }
    black
}

static NEIGHBORS: [Pos; 6] = [
    (0, -1, 1),
    (-1, 0, 1),
    (1, -1, 0),
    (-1, 1, 0),
    (0, 1, -1),
    (1, 0, -1),
];

pub fn part1(input: &str) -> usize {
    let paths = parse(input);
    let black = initialize(&paths);

    black.iter().count()
}

pub fn part2(input: &str) -> usize {
    let paths = parse(input);

    let mut last = initialize(&paths);

    let mut cnts = HashMap::new();

    for _ in 0..100 {
        let mut next = HashSet::new();
        for pos in &last {
            for (dx, dy, dz) in NEIGHBORS.iter() {
                *cnts
                    .entry((pos.0 + dx, pos.1 + dy, pos.2 + dz))
                    .or_insert(0) += 1;
            }
        }

        for (&tile_pos, &black_neighbors) in cnts.iter() {
            if last.contains(&tile_pos) {
                if black_neighbors == 1 || black_neighbors == 2 {
                    // this is a black tile staying black
                    next.insert(tile_pos);
                }
            } else {
                // this is a white tile
                if black_neighbors == 2 {
                    next.insert(tile_pos);
                }
            }
        }
        cnts.clear();
        last = next;
    }

    last.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day24() {
        let test_input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(part1(test_input), 10);
        assert_eq!(part2(&test_input), 2208);
    }

    #[test]
    fn day24() {
        let input = get_input(2020, 24).unwrap();
        assert_eq!(part1(&input), 386);
        assert_eq!(part2(&input), 4214);
    }
}
