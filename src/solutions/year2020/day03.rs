use super::*;
use std::collections::HashSet;

pub fn part1(input: &String) -> usize {
    let slope = (3, 1);
    let mut current = (0, 0);

    let (max_rows, max_cols, map) = parse(input);

    let mut trees = 0;
    while current.1 <= (max_rows + 1) {
        if map.contains(&current) {
            trees += 1;
        }
        current = ((current.0 + slope.0) % max_cols, current.1 + slope.1);
    }

    trees
}

pub fn part2(input: &String) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result = 1;
    for slope in slopes {
        let mut current = (0, 0);

        let (max_rows, max_cols, map) = parse(input);

        let mut trees = 0;
        while current.1 <= (max_rows + 1) {
            if map.contains(&current) {
                trees += 1;
            }
            current = ((current.0 + slope.0) % max_cols, current.1 + slope.1);
        }

        result *= trees;
    }

    result
}

pub fn parse(s: &String) -> (usize, usize, HashSet<(usize, usize)>) {
    let mut map = HashSet::new();
    let mut max_rows = 0;
    let mut max_cols = 0;
    for (i, line) in s.lines().enumerate() {
        for (j, position) in line.chars().enumerate() {
            if position == '#' {
                map.insert((j, i));
            }
            max_cols = j;
        }
        max_rows = i;
    }
    (max_rows + 1, max_cols + 1, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_test() {
        let test_input = "..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"
            .to_owned();
        assert_eq!(part1(&test_input), 7);
        assert_eq!(part2(&test_input), 336);
    }

    #[test]
    fn day02() {
        let input = get_input(2020, 3).unwrap();
        assert_eq!(part1(&input), 145);
        assert_eq!(part2(&input), 3424528800);
    }
}
