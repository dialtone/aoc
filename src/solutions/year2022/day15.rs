type Point = (i32, i32);

pub fn part1p(input: &str) -> usize {
    part1(input, 2_000_000)
}

pub fn part1e(input: &str) -> usize {
    part1(input, 10)
}

fn distance(p: Point, q: Point) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn parse(input: &str) -> Vec<(Point, Point, i32)> {
    let mut r = vec![];
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (sensor, beacon) = line.split_once(": ").unwrap();
        let (sx, sy) = &sensor[12..].split_once(", y=").unwrap();
        let (bx, by) = &beacon[23..].split_once(", y=").unwrap();
        let sens = (sx.parse().unwrap(), sy.parse().unwrap());
        let beac = (bx.parse().unwrap(), by.parse().unwrap());
        let dist = distance(sens, beac);

        r.push((sens, beac, dist));
    }

    r
}

pub fn part1(input: &str, y: usize) -> usize {
    let pairs = parse(input);
    println!("{:?}", pairs);
    0
}

pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn example() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(part1e(input), 26);
        // assert_eq!(part2(input), 93);
    }

    // #[test]
    // fn problem() {
    //     let input = get_input(2022, 14).unwrap();
    //     assert_eq!(part1(&input), 843);
    //     assert_eq!(part2(&input), 27625);
    // }
}
