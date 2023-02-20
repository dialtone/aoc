use rustc_hash::FxHashSet;

static MOVES: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn repl(c: u8) -> u8 {
    match c {
        b'S' => b'a',
        b'E' => b'z',
        _ => c,
    }
}
use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Copy, Clone, Eq, PartialEq)]
struct Explorer {
    nmoves: usize,
    pos: (usize, usize),
}

impl Ord for Explorer {
    fn cmp(&self, other: &Self) -> Ordering {
        other.nmoves.cmp(&self.nmoves)
    }
}

impl PartialOrd for Explorer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(input: &[u8], start_byte: u8, end_byte: u8) -> usize {
    let mut starts = vec![];
    let mut end = None;
    let hmap: Vec<&[u8]> = input
        .split(|c| c == &b'\n')
        .filter(|l| !l.is_empty())
        .collect();
    for (y, line) in hmap.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match *c {
                s if s == start_byte => starts.push((x, y)),
                e if e == end_byte => end = Some((x, y)),
                _ => continue,
            }
        }
    }
    let maxw = hmap[0].len();
    let maxh = hmap.len();

    let end = end.unwrap();

    let mut q = BinaryHeap::new();
    let mut seen: FxHashSet<(usize, usize)> = FxHashSet::default();
    for start in starts {
        seen.insert(start);
        q.push(Explorer {
            nmoves: 0,
            pos: start,
        });
    }

    // breadth first
    while let Some(Explorer { nmoves, pos }) = q.pop() {
        for d in MOVES {
            let newpos = (pos.0 as isize + d.0, pos.1 as isize + d.1);
            if newpos.0 < 0
                || newpos.0 >= maxw as isize
                || newpos.1 < 0
                || newpos.1 >= maxh as isize
            {
                // position out of map, skip
                continue;
            }
            let newpos = (newpos.0 as usize, newpos.1 as usize);

            if !seen.contains(&newpos)
                && (repl(hmap[newpos.1][newpos.0]) <= repl(hmap[pos.1][pos.0]) + 1)
            {
                // eligible since only 1 higher
                if newpos == end {
                    return nmoves + 1;
                }
                seen.insert(newpos);
                q.push(Explorer {
                    nmoves: nmoves + 1,
                    pos: newpos,
                });
            }
        }
    }
    0
}

pub fn part1(input: &[u8]) -> usize {
    search(input, b'S', b'E')
}

pub fn part2(input: &[u8]) -> usize {
    search(input, b'a', b'E')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day12_test() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(part1(input.as_bytes()), 31);
        assert_eq!(part2(input.as_bytes()), 29);
    }

    #[test]
    fn day12() {
        let input = get_input(2022, 12).unwrap();
        assert_eq!(part1(input.as_bytes()), 361);
        assert_eq!(part2(input.as_bytes()), 354);
    }
}
