// day11 parse             time:   [43.388 us 43.630 us 43.900 us]
// day11 part 1            time:   [7.3735 ms 7.4709 ms 7.5858 ms]
// day11 part 2            time:   [15.363 ms 15.437 ms 15.515 ms]

use super::*;
type Input = String;
type Parsed = Vec<Vec<char>>;

static ADJACIENTS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

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

fn next1(input: &Parsed, row: usize, col: usize) -> char {
    if input[row][col] == 'L' {
        if ADJACIENTS.iter().all(|&(r, c)| {
            if let Some(v) = input
                .get((r + row as isize) as usize)
                .and_then(|rr| rr.get((c + col as isize) as usize))
            {
                return *v == 'L' || *v == '.';
            }
            return true;
        }) {
            return '#';
        } else {
            return 'L';
        }
    } else {
        if ADJACIENTS
            .iter()
            .filter(|&(r, c)| {
                if let Some(v) = input
                    .get((r + row as isize) as usize)
                    .and_then(|row| row.get((c + col as isize) as usize))
                {
                    return *v == '#';
                }
                return false;
            })
            .count()
            >= 4
        {
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

fn next2(input: &Parsed, row: usize, col: usize) -> char {
    if input[row][col] == 'L' {
        if ADJACIENTS.iter().all(|&(r, c)| {
            let mut t_row = (r + row as isize) as usize;
            let mut t_col = (c + col as isize) as usize;

            while let Some(v) = input.get(t_row).and_then(|rr| rr.get(t_col)) {
                if *v == '.' {
                    t_row = (r + t_row as isize) as usize;
                    t_col = (c + t_col as isize) as usize;
                } else if *v == 'L' {
                    return true;
                } else {
                    return false;
                }
            }
            return true;
        }) {
            return '#';
        } else {
            return 'L';
        }
    } else {
        if ADJACIENTS
            .iter()
            .filter(|&(r, c)| {
                let mut t_row = (r + row as isize) as usize;
                let mut t_col = (c + col as isize) as usize;

                while let Some(v) = input.get(t_row).and_then(|rr| rr.get(t_col)) {
                    if *v == '.' {
                        t_row = (r + t_row as isize) as usize;
                        t_col = (c + t_col as isize) as usize;
                    } else if *v == '#' {
                        return true;
                    } else {
                        return false;
                    }
                }
                return false;
            })
            .count()
            >= 5
        {
            return 'L';
        } else {
            return '#';
        }
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
