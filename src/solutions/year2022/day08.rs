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

static RIGHT: (i8, i8) = (1, 0);
static LEFT: (i8, i8) = (-1, 0);
static UP: (i8, i8) = (0, -1);
static DOWN: (i8, i8) = (0, 1);

pub fn part2(input: &str) -> usize {
    let field = parse(input);
    let visible = find_candidates(&field);

    let mut highest_distance = 0;
    for candidate in visible {
        let height = field[candidate.1][candidate.0];
        let (mut x, mut y) = candidate;
        let mut distances = vec![];
        for direction in [UP, DOWN, LEFT, RIGHT].iter() {
            let mut distance = 0;
            loop {
                if x == 0 || y == 0 || x == field[0].len() - 1 || y == field.len() - 1 {
                    x = candidate.0;
                    y = candidate.1;
                    break;
                }
                distance += 1;
                x = (x as i8 + direction.0) as usize;
                y = (y as i8 + direction.1) as usize;
                if height > field[y][x] {
                    continue;
                } else {
                    x = candidate.0;
                    y = candidate.1;
                    break;
                }
            }
            distances.push(distance);
        }
        let new_score = distances.iter().product();
        if new_score > highest_distance {
            highest_distance = new_score;
        }
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
