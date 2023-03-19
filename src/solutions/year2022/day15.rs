use rustc_hash::FxHashSet;

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

pub fn parse(input: &str) -> Vec<(Point, Point, i32)> {
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

// this is for the other slower alternatives
// pub fn parse(input: &str) -> (Vec<(Point, Point, i32)>, Point, Point) {
//     let mut r = vec![];
//     let (mut minx, mut maxx, mut miny, mut maxy) = (0, 0, 0, 0);
//
//     for line in input.lines().filter(|l| !l.is_empty()) {
//         let (sensor, beacon) = line.split_once(": ").unwrap();
//         let (sx, sy) = &sensor[12..].split_once(", y=").unwrap();
//         let (bx, by) = &beacon[23..].split_once(", y=").unwrap();
//         let sens = (sx.parse().unwrap(), sy.parse().unwrap());
//         let beac = (bx.parse().unwrap(), by.parse().unwrap());
//         let dist = distance(sens, beac);
//
//         minx = minx.min(sens.0 - dist);
//         minx = minx.min(beac.0);
//
//         maxx = maxx.max(sens.0 + dist);
//         maxx = maxx.max(beac.0);
//
//         miny = miny.min(sens.1 - dist);
//         miny = miny.min(beac.1);
//
//         maxy = maxy.max(sens.1 + dist);
//         maxy = maxy.max(beac.1);
//
//         r.push((sens, beac, dist));
//     }
//
//     (r, (minx, miny), (maxx, maxy))
// }

pub fn intersect(s: Point, d: i32, yline: i32) -> Option<(i32, i32)> {
    let remainder = d - (s.1 - yline).abs();
    if remainder >= 0 {
        return Some(((s.0 - remainder), 1 + 2 * remainder));
    }
    None
}

// 08765432101234567890123456789012345678
// 0..........#..........................
// 9.........###.........................
// 8........#####........................
// 7.......#######.......................
// 6......#########.............#........
// 5.....###########...........###.......
// 4....#############.........#####......
// 3...###############.......#######.....
// 2..#################.....#########....
// 1.###################.#.###########...
// 0##########S########################..
// 1.###########################S#######.
// 2..###################S#############..
// 3...####################B##########...
// 4....#############################....
// 5.....###########################.....
// 6......#########################......
// 7.......#########S#############.......
// 8........#######################......
// 9.......#########################.....
// 0......####B######################....
// 1.....###S#############.###########...
// 2......#############################..
// 3.......#############################.
// 4.......#############S#######S########
// 5......B#############################.
// 6.....############SB################..
// 7....##################S##########B...
// 8...#######S######################....
// 9....############################.....
// 0.....#############S######S######.....
// 1......#########################......
// 2.......#######..#############B.......
// 3........#####....###..#######........
// 4.........###......#....#####.........
// 5..........#.............###..........
// 6.........................#...........

// 0012345678901234567890
// 0##S##################
// 1####################S
// 2#############S#######
// 3###############B#####
// 4#####################
// 5#####################
// 6#####################
// 7########S############
// 8#####################
// 9#####################
// 0##B##################
// 1S#############.######
// 2#####################
// 3#####################
// 4############S#######S
// 5#####################
// 6#########SB##########
// 7##############S######
// 8##S##################
// 9#####################
// 0##########S######S###

// the concept of this idea is that the distance of the sensor from the reference line
// implies a certain amount of overlap, correlated with the distance, centered around the
// sensor's x coord. This way we just need to check the y distance between line and sensor
// and the whole thing is done with a handful of operations, solution here takes 0.2us with parsing
// taking 3.5us
// year 22 day15 part 1    time:   [3.7573 µs 3.7663 µs 3.7782 µs]
pub fn part1(input: &str, y: i32) -> usize {
    let pairs = parse(input);
    let mut removals: FxHashSet<i32> = FxHashSet::default();

    let mut intervals = Vec::with_capacity(pairs.len());

    for (s, b, d) in pairs.iter().filter(|(s, _, d)| (s.1 - y).abs() < *d) {
        if let Some((start, length)) = intersect(*s, *d, y) {
            intervals.push((start, start + length - 1));
        }

        if b.1 == y {
            removals.insert(b.0);
        }
        if s.1 == y {
            removals.insert(s.0);
        }
    }
    intervals.sort();

    let mut overlaps = vec![intervals[0]];

    for (start, stop) in intervals[1..].iter() {
        if let Some(last) = overlaps.last_mut() {
            if last.1 >= *start {
                *last = (last.0.min(*start), *stop.max(&last.1));
            } else {
                overlaps.push((*start, *stop));
            }
        } else {
            overlaps.push((*start, *stop));
        }
    }

    overlaps
        .iter()
        .map(|&(start, stop)| stop - start + 1)
        .sum::<i32>() as usize
        - removals.len()

    // attempt 2 900us
    // auto-vectorizer does a lot of heavy lifting here, but still slower
    // year 22 day15 part 1    time:   [902.87 µs 909.07 µs 915.43 µs]
    //
    // let (pairs, topleft, botright) = parse(input);
    //
    // let addx = topleft.0.abs();
    //
    // let mut line = vec![false; (botright.0 - topleft.0 + 1) as usize];
    //
    // for (s, b, d) in pairs.iter().filter(|(s, _, d)| (s.1 - y).abs() < *d) {
    //     if let Some((start, length)) = intersect(*s, *d, y) {
    //         for i in addx + start..(addx + start + length) {
    //             line[i as usize] = true;
    //         }
    //     }
    //     if b.1 == y {
    //         line[(b.0 + addx) as usize] = false
    //     }
    //     if s.1 == y {
    //         line[(s.0 + addx) as usize] = false
    //     }
    // }
    //
    // line.iter().filter(|&&v| v).count();

    // attempt 1 4-5seconds
    // let mut no_source = 0;
    // // dbg!(topleft.0, botright.0);
    // //
    // // print!("{}", 0);
    // // for x in topleft.0..=botright.0 {
    // //     if x < 0 || x > 20 {
    // //         continue;
    // //     }
    // //     print!("{}", x.abs() % 10);
    // // }
    // // println!();
    //
    // // for y in topleft.1..=botright.1 {
    // //     if y < 0 || y > 20 {
    // //         continue;
    // //     }
    // //     print!("{}", y.abs() % 10);
    //
    // // let squares: Vec<(Point, Point)> = pairs
    // //     .iter()
    // //     .map(|(s, _, d)| ((s.0 - d, s.1 - d), (s.0 + d, s.1 + d)))
    // //     .collect();
    //
    // // let pairs: Vec<(Point, Point, i32)> = pairs
    // //     .into_iter()
    // //     .filter(|(s, _, d)| (s.1 - y).abs() < *d)
    // //     .collect();
    //
    // for x in topleft.0..=botright.0 {
    //     // if x < 0 || x > 20 {
    //     //     continue;
    //     // }
    //     // let mut ch = ".";
    //     for (s, b, d) in pairs.iter() {
    //         let d2 = distance(*s, (x, y));
    //         // if *b == (x, y) {
    //         //     ch = "B";
    //         //     break;
    //         // } else if *s == (x, y) {
    //         //     ch = "S";
    //         //     break;
    //         // } else
    //         if *d >= d2 && *b != (x, y) {
    //             // print!("#");
    //             // ch = "#";
    //             no_source += 1;
    //             break;
    //         } // else {
    //           // ch = ".";
    //           // print!(".");
    //           //}
    //     }
    //     // print!("{}", ch);
    // }
    // //     println!()
    // // }
    // // println!();
    // no_source
}

// old version with an extra vec allocation
// year 22 day15 part 2    time:   [250.90 ms 251.17 ms 251.48 ms]
// current version without extra vec allocation
// year 22 day15 part 2    time:   [176.76 ms 176.99 ms 177.27 ms]
pub fn part2(input: &str, limit: i32) -> usize {
    let pairs = parse(input);

    let mut intervals = Vec::with_capacity(pairs.len());
    for y in 0..=limit {
        intervals.clear();
        for (s, _, d) in pairs.iter().filter(|(s, _, d)| (s.1 - y).abs() < *d) {
            if let Some((start, length)) = intersect(*s, *d, y) {
                intervals.push((start, start + length - 1));
            }
        }
        intervals.sort();

        let mut last = intervals[0];
        let mut num = 0;
        for (start, stop) in intervals[1..].iter() {
            if last.1 >= *start && last.0 <= *stop {
                last = (last.0.min(*start).max(0), *stop.max(&last.1).min(&limit));
            } else {
                num += 1;
                last = (*start, *stop);
            }
        }

        if num == 1 {
            return (last.0 as usize - 1) * 4000000 + y as usize;
        }
    }

    0
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
        assert_eq!(part2(input, 20), 56000011);
    }

    #[test]
    fn problem() {
        let input = get_input(2022, 15).unwrap();
        assert_eq!(part1p(&input), 4883971);
        assert_eq!(part2(&input, 4_000_000), 12691026767556);
    }
}
