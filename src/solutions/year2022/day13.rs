use itertools::Itertools;
use std::cmp::Ordering;

fn compare(left: &&[u8], right: &&[u8]) -> Ordering {
    compare_(left, right, 0, 0)
}

fn compare_(left: &&[u8], right: &&[u8], wrapped_left: u8, wrapped_right: u8) -> Ordering {
    if left.is_empty() && right.is_empty() {
        return Ordering::Equal;
    }
    if left.is_empty() {
        return Ordering::Less;
    }
    if right.is_empty() {
        return Ordering::Greater;
    }
    match (left[0], right[0]) {
        // when we wrap numbers we align lists by force, so we end up at a place where we have a
        // next , to determine next item, if we get to that spot but are wrapped we need to return
        // because we can tell what's bigger and smaller already.
        (b',', b',') if wrapped_left > 0 => Ordering::Less,
        (b',', b',') if wrapped_right > 0 => Ordering::Greater,

        // same, consume strings (this could be any byte include the numbers we compare)
        (x, y) if x == y => compare_(&&left[1..], &&right[1..], wrapped_left, wrapped_right),

        // closed list on left, check if we got wrapped but then resolve
        (b']', x) if x != b']' => {
            if wrapped_right > 0 {
                compare_(&&left[1..], right, wrapped_left, wrapped_right - 1)
            } else {
                Ordering::Less
            }
        }
        // closed list on right, check if we got wrapped and then resolve
        (x, b']') if x != b']' => {
            if wrapped_left > 0 {
                compare_(left, &&right[1..], wrapped_left - 1, wrapped_right)
            } else {
                Ordering::Greater
            }
        }

        (b',', _) => Ordering::Less,
        (_, b',') => Ordering::Greater,

        // we have a comparison with a list but the item on the other side isn't a list, so it
        // needs to be wrapped in one, and let's keep track of it.
        (b'[', x) if x != b'[' => compare_(&&left[1..], right, wrapped_left, wrapped_right + 1),
        (x, b'[') if x != b'[' => compare_(left, &&right[1..], wrapped_left + 1, wrapped_right),

        // we're comparing real numbers here, however we need to account for the 10, so we look
        // ahead a bit
        (x, y) if x.is_ascii_digit() && y.is_ascii_digit() => {
            match (left[1].is_ascii_digit(), right[1].is_ascii_digit()) {
                (true, true) => compare_(&&left[2..], &&right[2..], wrapped_left, wrapped_right),
                (false, true) => Ordering::Less,
                (true, false) => Ordering::Greater,
                (false, false) => {
                    if x < y {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }
        }
        _ => unreachable!(),
    }
}

fn tostring(s: &[u8]) -> String {
    s.iter().map(|c| *c as char).collect()
}

// year 22 day13 part 1    time:   [11.120 µs 11.132 µs 11.145 µs] (recursive)
pub fn part1(input: &[u8]) -> usize {
    let mut ans = 0;
    for (i, (leftt, right)) in input
        .split(|c| c == &b'\n')
        .filter(|l| !l.is_empty())
        .tuples()
        .enumerate()
    {
        if compare(&leftt, &right) == Ordering::Less {
            ans += i + 1;
        }
    }
    ans
}

pub fn part2(input: &[u8]) -> usize {
    let mut v = input
        .split(|c| c == &b'\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<&[u8]>>();

    v.push("[[2]]".as_bytes());
    v.push("[[6]]".as_bytes());
    v.sort_by(compare);

    let mut r = 1;
    for (i, item) in v.iter().enumerate() {
        if *item == "[[2]]".as_bytes() {
            r *= i + 1;
        }
        if *item == "[[6]]".as_bytes() {
            r *= i + 1;
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn example() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(part1(input.as_bytes()), 13);
        assert_eq!(part2(input.as_bytes()), 140);

        //
        let input = "[1,1,3,1,1]
[1,1,5,1,10]";
        assert_eq!(part1(input.as_bytes()), 1);

        let input = "[[3,[1,8],0]]
[[[3],[[6,9,9]],4,[[],8],9],[[[1,0,7],[2,1,4],[0,9,4,10,2]],[[]],1,1],[[8,10,7],[6],5],[4]]

[[[],[0,3,2],[]],[],[6,[[],1,[10,1,0,6],[]],10,5],[]]
[[],[[2,2,[3,10,1,5]],[7,5],[[],[4,4,10,3],6,[7,6,5],[2]],[[10,3],[6,9],1],6],[[0,4,3,[7,1,9]],0,4],[10],[[[7,9,2,7,6],[]],1,[1,[4],[],[0,2,9,5,9]],5,[9]]]

[[8,[6],[[9,6,3,3,4],2,[],5],[7,10]],[],[8,[4],4,8]]
[[8],[9,9,[[9,0,9,2,9],[6,3,7,7]],[],[[1,3,9,8,6],[9]]]]";
        assert_eq!(part1(input.as_bytes()), 1);

        let input = "[[[4,[1],10],1,[[3,10],[4]],[],[10,10,6,9,[0,2]]],[9,[4,9,[4,0,8,0],[]]],[[[8],[5,2,1],[8,5,0,6],[6,8,5],6],[[],[]],[6,[7,7],[7,7,4,4,4],[8,2]]],[]]
[[4,2,9],[[]],[4,8,2],[[10,7,2,[1,5,4,1]]],[]]";
        assert_eq!(part1(input.as_bytes()), 0);

        let input = "[[6,4,7,[]],[3,[5,[10,3,3],3,8,0],7,[0,[2,5,9],4,[9]]],[8,0,[[7,0,6,6],[7,6,6,7,1],[2,7,4,7]]]]
[[[6,[2,2,0,5,2],1,[10,8],5],7,[[],[10,9,0,7,9],2,5],[[6,8,4,3],[],5]],[0,[2,4],7],[[4,3,[7],2,9],9,9]]";
        assert_eq!(part1(input.as_bytes()), 1);
    }

    #[test]
    fn problem() {
        let input = get_input(2022, 13).unwrap();
        // 5577 too high
        // 5526 too low
        // 5504 too low
        assert_eq!(part1(input.as_bytes()), 5555);
        assert_eq!(part2(input.as_bytes()), 354);
    }
}

// [src/solutions/year2022/day13.rs:61] i = 50
// [src/solutions/year2022/day13.rs:63] tostring(leftt) = ""
// [src/solutions/year2022/day13.rs:64] tostring(right) = ""
// [src/solutions/year2022/day13.rs:65] 0 = 0

// [[[4,[1],10],1,[[3,10],[4]],[],[10,10,6,9,[0,2]]],[9,[4,9,[4,0,8,0],[]]],[[[8],[5,2,1],[8,5,0,6],[6,8,5],6],[[],[]],[6,[7,7],[7,7,4,4,4],[8,2]]],[]]
// [[4,2,9],[[]],[4,8,2],[[10,7,2,[1,5,4,1]]],[]]

//
// [[3,[1,8],0]]
// [[[3],[[6,9,9]],4,[[],8],9],[[[1,0,7],[2,1,4],[0,9,4,10,2]],[[]],1,1],[[8,10,7],[6],5],[4]]
//
// [[[],[0,3,2],[]],[],[6,[[],1,[10,1,0,6],[]],10,5],[]]
// [[],[[2,2,[3,10,1,5]],[7,5],[[],[4,4,10,3],6,[7,6,5],[2]],[[10,3],[6,9],1],6],[[0,4,3,[7,1,9]],0,4],[10],[[[7,9,2,7,6],[]],1,[1,[4],[],[0,2,9,5,9]],5,[9]]]
//
// [[8,[6],[[9,6,3,3,4],2,[],5],[7,10]],[],[8,[4],4,8]]
// [[8],[9,9,[[9,0,9,2,9],[6,3,7,7]],[],[[1,3,9,8,6],[9]]]]
