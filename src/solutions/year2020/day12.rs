use super::*;

type Input = String;
type Parsed = Vec<Move>;

pub enum Move {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

static DIRECTION: [fn(usize) -> Move; 4] = [Move::East, Move::South, Move::West, Move::North];

pub struct Point {
    pub x: isize,
    pub y: isize,
    pub orientation: usize,
}

impl Point {
    fn new(x: isize, y: isize, orientation: usize) -> Self {
        Point { x, y, orientation }
    }

    fn next(&mut self, m: &Move) {
        match m {
            &Move::North(amount) => self.y += amount as isize,
            &Move::South(amount) => self.y -= amount as isize,
            &Move::East(amount) => self.x += amount as isize,
            &Move::West(amount) => self.x -= amount as isize,
            &Move::Left(degrees) => {
                if self.orientation == 0 {
                    let x = self.x;
                    let y = self.y;

                    if degrees == 90 {
                        self.x = -y;
                        self.y = x;
                    } else if degrees == 180 {
                        self.x = -x;
                        self.y = -y;
                    } else if degrees == 270 {
                        self.x = y;
                        self.y = -x;
                    } else {
                        unreachable!()
                    }
                } else {
                    let increments = degrees / 90;
                    self.orientation -= increments;
                }
            }
            &Move::Right(degrees) => {
                if self.orientation == 0 {
                    let x = self.x;
                    let y = self.y;
                    let x = self.x;
                    let y = self.y;

                    if degrees == 90 {
                        self.y = -x;
                        self.x = y;
                    } else if degrees == 180 {
                        self.x = -x;
                        self.y = -y;
                    } else if degrees == 270 {
                        self.x = -y;
                        self.y = x;
                    } else {
                        unreachable!()
                    }
                } else {
                    let increments = degrees / 90;
                    self.orientation += increments;
                }
            }
            &Move::Forward(amount) => self.next(&DIRECTION[self.orientation % 4](amount)),
        }
    }
}

pub struct Point2 {
    pub x: isize,
    pub y: isize,
    pub waypoint: Point,
}

impl Point2 {
    fn new(x: isize, y: isize, waypoint: Point) -> Self {
        Point2 { x, y, waypoint }
    }

    fn next(&mut self, m: &Move) {
        match m {
            &Move::North(amount) => self.waypoint.next(m),
            &Move::South(amount) => self.waypoint.next(m),
            &Move::East(amount) => self.waypoint.next(m),
            &Move::West(amount) => self.waypoint.next(m),
            &Move::Left(degrees) => {
                self.waypoint.next(m);
            }
            &Move::Right(degrees) => {
                self.waypoint.next(m);
            }
            &Move::Forward(amount) => {
                self.x += self.waypoint.x * amount as isize;
                self.y += self.waypoint.y * amount as isize;
            }
        }
    }
}

pub fn part1(input: &Parsed) -> isize {
    let mut position = Point::new(0, 0, 400);
    for m in input {
        position.next(m);
    }
    position.x.abs() + position.y.abs()
}

pub fn part2(input: &Parsed) -> isize {
    let mut position = Point2::new(0, 0, Point::new(10, 1, 0));
    for m in input {
        position.next(m);
    }
    position.x.abs() + position.y.abs()
}

pub fn parse(s: &Input) -> Parsed {
    s.lines()
        .map(|l| {
            let (m, size) = l.split_at(1);
            to_move(m, size.parse().unwrap())
        })
        .collect()
}

fn to_move(c: &str, n: usize) -> Move {
    match c {
        "N" => Move::North(n),
        "S" => Move::South(n),
        "E" => Move::East(n),
        "W" => Move::West(n),
        "L" => Move::Left(n),
        "R" => Move::Right(n),
        "F" => Move::Forward(n),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12() {
        let test_input = "F10
N3
F7
R90
F11"
        .to_owned();
        assert_eq!(part1(&parse(&test_input)), 25);
        assert_eq!(part2(&parse(&test_input)), 286);
    }

    #[test]
    fn day12() {
        let input = get_input(2020, 12).unwrap();
        assert_eq!(part1(&parse(&input)), 757);
        assert_eq!(part2(&parse(&input)), 51249);
    }
}
