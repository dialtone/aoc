use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Vec<i8>> {
    let mut field: Vec<Vec<i8>> = vec![];
    input
        .lines()
        .for_each(|line| field.push(line.chars().map(|c| (c as u8 - b'0') as i8).collect()));
    field
}

pub fn find_candidates(field: &[Vec<i8>]) -> HashSet<(usize, usize)> {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for x in 0..field[0].len() {
        // down
        let mut current_height = -1;
        for (y, row) in field.iter().enumerate() {
            if row[x] > current_height {
                visible.insert((x, y));
                current_height = row[x];
            }
        }
        // up
        current_height = -1;
        for (y, row) in field.iter().enumerate().rev() {
            if row[x] > current_height {
                visible.insert((x, y));
                current_height = row[x];
            }
        }
    }

    for (y, line) in field.iter().enumerate() {
        // right
        line.iter().enumerate().fold(-1, |height, (x, tree)| {
            if tree > &height {
                visible.insert((x, y));
                *tree
            } else {
                height
            }
        });

        // left
        line.iter().enumerate().rev().fold(-1, |height, (x, tree)| {
            if tree > &height {
                visible.insert((x, y));
                *tree
            } else {
                height
            }
        });
    }

    visible
}

pub fn part1(input: &str) -> usize {
    let field = parse(input);
    let visible = find_candidates(&field);
    visible.len()
}

pub fn part2(input: &str) -> usize {
    let field = parse(input);
    let visible = find_candidates(&field);
    let s = field.len();
    let mut highest_distance = 0;
    for candidate in visible {
        let height = field[candidate.1][candidate.0];
        let (x, y) = candidate;
        if x == 0 || y == 0 || x == s - 1 || y == s - 1 {
            continue;
        }
        let mut acc = 1;
        // left
        for (distance, tree_pos) in (0..x).rev().enumerate() {
            if height <= field[y][tree_pos] {
                acc *= distance + 1;
                break;
            }
            if tree_pos == 0 {
                acc *= distance + 1;
            }
        }

        // right
        for (distance, tree_pos) in (x + 1..s).enumerate() {
            if height <= field[y][tree_pos] {
                acc *= distance + 1;
                break;
            }
            if tree_pos == s - 1 {
                acc *= distance + 1;
            }
        }
        // up
        for (distance, tree_pos) in (0..y).rev().enumerate() {
            if height <= field[tree_pos][x] {
                acc *= distance + 1;
                break;
            }
            if tree_pos == 0 {
                acc *= distance + 1;
            }
        }
        // down
        for (distance, tree_pos) in (y + 1..s).enumerate() {
            if height <= field[tree_pos][x] {
                acc *= distance + 1;
                break;
            }
            if tree_pos == s - 1 {
                acc *= distance + 1;
            }
        }

        highest_distance = highest_distance.max(acc);
    }
    highest_distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day08_test() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(part1(input), 21);
        assert_eq!(part2(input), 8);
    }

    #[test]
    fn day08() {
        let input = get_input(2022, 8).unwrap();
        assert_eq!(part1(&input), 1829);
        assert_eq!(part2(&input), 291840);
    }
}
