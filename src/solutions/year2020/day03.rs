pub fn part1(map: &Vec<Vec<bool>>) -> usize {
    let slope = (3, 1);
    count_trees(map, slope)
}

fn count_trees(map: &Vec<Vec<bool>>, slope: (usize, usize)) -> usize {
    let max_cols = map[0].len();
    let max_rows = map.len();
    let mut current = (0, 0);

    let mut trees = 0;
    while current.1 < max_rows {
        if map[current.1][current.0] {
            trees += 1;
        }
        current = ((current.0 + slope.0) % max_cols, current.1 + slope.1);
    }
    trees
}

pub fn part2(map: &Vec<Vec<bool>>) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result = 1;
    for slope in slopes {
        result *= count_trees(map, slope);
    }

    result
}

pub fn parse(s: &String) -> Vec<Vec<bool>> {
    let mut map = Vec::with_capacity(s.lines().count());
    for line in s.lines() {
        let mut row = Vec::with_capacity(line.len());
        for position in line.chars() {
            if position == '#' {
                row.push(true);
            } else {
                row.push(false);
            }
        }
        map.push(row);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day03_test() {
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
        assert_eq!(part1(&parse(&test_input)), 7);
        assert_eq!(part2(&parse(&test_input)), 336);
    }

    #[test]
    fn day03() {
        let input = get_input(2020, 3).unwrap();
        assert_eq!(part1(&parse(&input)), 145);
        assert_eq!(part2(&parse(&input)), 3424528800);
    }
}
