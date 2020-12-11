// day11 parse             time:   [43.227 us 43.738 us 44.376 us]
// day11 part 1            time:   [41.884 ms 42.233 ms 42.645 ms]
// day11 part 2            time:   [54.527 ms 54.802 ms 55.098 ms]

use super::*;
type Input = String;
type Parsed = Vec<Vec<char>>;

fn printmap(m: &Parsed) {
    for line in m {
        println!("{}", line.iter().collect::<String>());
    }
    println!("---")
}

pub fn part1(oinput: &Parsed) -> usize {
    let mut input = oinput.clone();
    let cols = input[0].len();
    let rows = input.len();
    let mut last_occupied = 50000;
    loop {
        let mut new_input = input.clone();
        for row in 0..rows {
            for col in 0..cols {
                if new_input[row][col] == '.' {
                    continue;
                }
                new_input[row][col] = next1(&input, row, col);
            }
        }
        input = new_input;
        let occupied = input
            .iter()
            .map(|row| row.iter().filter(|c| **c == '#').count())
            .sum();
        if last_occupied == occupied {
            return occupied;
        }
        last_occupied = occupied;
        // printmap(&input);
    }
}

fn next2(input: &Parsed, row: usize, col: usize) -> char {
    let adjacients: Vec<(i32, i32)> = if row == 0 {
        if col == 0 {
            vec![(1, 0), (1, 1), (0, 1)]
        } else if col < input[0].len() - 1 {
            vec![(0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        } else {
            vec![(0, -1), (1, -1), (1, 0)]
        }
    } else if row < input.len() - 1 {
        if col == 0 {
            vec![(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0)]
        } else if col < input[0].len() - 1 {
            vec![
                (0, -1),
                (0, 1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ]
        } else {
            assert!(col == input[0].len() - 1);
            vec![(0, -1), (1, -1), (1, 0), (-1, 0), (-1, -1)]
        }
    } else {
        assert!(row == input.len() - 1);
        if col == 0 {
            vec![(0, 1), (0 - 1, 1), (0 - 1, 0)]
        } else if col < input[0].len() - 1 {
            vec![(0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1)]
        } else {
            vec![(0, -1), (-1, -1), (-1, 0)]
        }
    };

    if input[row][col] == 'L' {
        let next_occupied = adjacients.iter().all(|(r, c)| {
            let mut t_row = (*r) + row as i32;
            let mut t_col = (*c) + col as i32;
            let mut adjacient_cell = input[t_row as usize][t_col as usize];
            if adjacient_cell == 'L' {
                return true;
            } else if adjacient_cell == '.' {
                while (t_row >= 0
                    && t_row <= (input.len() - 1) as i32
                    && t_col >= 0
                    && t_col <= (input[0].len() - 1) as i32)
                    && adjacient_cell == '.'
                {
                    adjacient_cell = input[t_row as usize][t_col as usize];
                    t_col = *c + t_col as i32;
                    t_row = *r + t_row as i32;
                }
                return adjacient_cell == 'L' || adjacient_cell == '.';
            } else {
                return false; // this is a # cell
            }
        });
        if next_occupied {
            return '#';
        } else {
            return 'L';
        }
    } else {
        let next_empty = adjacients
            .iter()
            .filter(|(r, c)| {
                let mut t_row = (*r) + row as i32;
                let mut t_col = (*c) + col as i32;
                let mut adjacient_cell = input[t_row as usize][t_col as usize];
                if adjacient_cell == '#' {
                    return true;
                } else if adjacient_cell == '.' {
                    while (t_row >= 0
                        && t_row <= (input.len() - 1) as i32
                        && t_col >= 0
                        && t_col <= (input[0].len() - 1) as i32)
                        && adjacient_cell == '.'
                    {
                        adjacient_cell = input[t_row as usize][t_col as usize];
                        t_col = *c + t_col as i32;
                        t_row = *r + t_row as i32;
                    }
                    return adjacient_cell == '#';
                } else {
                    return false; // this is an L cell
                }
            })
            .count()
            >= 5;
        if next_empty {
            return 'L';
        } else {
            return '#';
        }
    }
}

fn next1(input: &Parsed, row: usize, col: usize) -> char {
    let adjacients = if row == 0 {
        if col == 0 {
            vec![(1, 0), (1, 1), (0, 1)]
        } else if col < input[0].len() - 1 {
            vec![
                (0, col - 1),
                (0, col + 1),
                (1, col - 1),
                (1, col),
                (1, col + 1),
            ]
        } else {
            vec![(0, col - 1), (1, col - 1), (1, col)]
        }
    } else if row < input.len() - 1 {
        if col == 0 {
            vec![
                (row - 1, 0),
                (row - 1, 1),
                (row, 1),
                (row + 1, 1),
                (row + 1, 0),
            ]
        } else if col < input[0].len() - 1 {
            vec![
                (row, col - 1),
                (row, col + 1),
                (row + 1, col),
                (row - 1, col),
                (row + 1, col + 1),
                (row + 1, col - 1),
                (row - 1, col + 1),
                (row - 1, col - 1),
            ]
        } else {
            assert!(col == input[0].len() - 1);
            vec![
                (row, col - 1),
                (row + 1, col - 1),
                (row + 1, col),
                (row - 1, col),
                (row - 1, col - 1),
            ]
        }
    } else {
        assert!(row == input.len() - 1);
        if col == 0 {
            vec![(row, 1), (row - 1, 1), (row - 1, 0)]
        } else if col < input[0].len() - 1 {
            vec![
                (row, col - 1),
                (row, col + 1),
                (row - 1, col - 1),
                (row - 1, col),
                (row - 1, col + 1),
            ]
        } else {
            vec![(row, col - 1), (row - 1, col - 1), (row - 1, col)]
        }
    };

    if input[row][col] == 'L' {
        let next_occupied = adjacients
            .iter()
            .all(|(r, c)| input[*r][*c] == 'L' || input[*r][*c] == '.');
        if next_occupied {
            return '#';
        } else {
            return 'L';
        }
    } else {
        let next_empty = adjacients
            .iter()
            .filter(|(r, c)| input[*r][*c] == '#')
            .count()
            >= 4;
        if next_empty {
            return 'L';
        } else {
            return '#';
        }
    }
}

pub fn part2(oinput: &Parsed) -> usize {
    let mut input = oinput.clone();
    let cols = input[0].len();
    let rows = input.len();
    let mut last_occupied = 50000;
    loop {
        let mut new_input = input.clone();
        for row in 0..rows {
            for col in 0..cols {
                if new_input[row][col] == '.' {
                    continue;
                }
                new_input[row][col] = next2(&input, row, col);
            }
        }
        input = new_input;
        let occupied = input
            .iter()
            .map(|row| row.iter().filter(|c| **c == '#').count())
            .sum();
        if last_occupied == occupied {
            return occupied;
        }
        last_occupied = occupied;
        // printmap(&input);
    }
}

pub fn parse(s: &Input) -> Parsed {
    let mut map = Vec::new();
    for line in s.lines() {
        map.push(line.chars().collect::<Vec<char>>());
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09() {
        let test_input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .to_owned();
        assert_eq!(part1(&parse(&test_input)), 37);
        assert_eq!(part2(&parse(&test_input)), 26);
    }

    #[test]
    fn day09() {
        let input = get_input(2020, 11).unwrap();
        assert_eq!(part1(&parse(&input)), 2204);
        assert_eq!(part2(&parse(&input)), 1986);
    }
}
