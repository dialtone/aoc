// day12 parse             time:   [13.672 us 13.772 us 13.887 us]
// day12 part 1            time:   [6.9036 us 7.0104 us 7.1657 us]
// day12 part 2            time:   [6.4478 us 6.4916 us 6.5392 us]

use super::*;

type Input = String;
type Move = (u8, isize);
type Parsed = Vec<Move>;

static DIRECTION: [u8; 4] = [b'E', b'S', b'W', b'N'];

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
            &(b'N', amount) => self.y += amount as isize,
            &(b'S', amount) => self.y -= amount as isize,
            &(b'E', amount) => self.x += amount as isize,
            &(b'W', amount) => self.x -= amount as isize,
            &(b'L', degrees) => {
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
                    self.orientation -= increments as usize;
                }
            }
            &(b'R', degrees) => {
                if self.orientation == 0 {
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
                    self.orientation += increments as usize;
                }
            }
            &(b'F', amount) => self.next(&(DIRECTION[self.orientation % 4], amount)),
            _ => unreachable!(),
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
            &(b'N', _) => self.waypoint.next(m),
            &(b'S', _) => self.waypoint.next(m),
            &(b'E', _) => self.waypoint.next(m),
            &(b'W', _) => self.waypoint.next(m),
            &(b'L', _) => {
                self.waypoint.next(m);
            }
            &(b'R', _) => {
                self.waypoint.next(m);
            }
            &(b'F', amount) => {
                self.x += self.waypoint.x * amount as isize;
                self.y += self.waypoint.y * amount as isize;
            }
            _ => {
                unreachable!()
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
        .map(|l| (l.as_bytes()[0], l[1..].parse().unwrap()))
        .collect()
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
