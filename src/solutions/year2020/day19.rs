// day19 part 1            time:   [6.6383 ms 6.7153 ms 6.7967 ms]
// day19 part 2            time:   [16.452 ms 16.654 ms 16.889 ms]
use super::*;
use std::collections::*;

#[derive(Debug, Clone)]
pub enum Node {
    Either(Vec<usize>, Vec<usize>),
    Seq(Vec<usize>),
    Letter(char),
}

fn parse(input: &str) -> (&str, HashMap<usize, Node>) {
    let mut map = HashMap::new();
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();

    for line in rules.lines() {
        let mut parts = line.split(": ");
        let rule_id = parts.next().unwrap().parse().unwrap();
        let rule_body = parts.next().unwrap();
        if rule_body.contains(&" | ") {
            let body = rule_body.split(" | ");
            let mut sides = body.map(|side| side.split(" ").map(|x| x.parse().unwrap()).collect());
            let node = Node::Either(sides.next().unwrap(), sides.next().unwrap());
            map.insert(rule_id, node);
        } else if rule_body.contains(&"\"") {
            let letter = rule_body.chars().filter(|&x| x != '"').next().unwrap();
            map.insert(rule_id, Node::Letter(letter));
        } else {
            map.insert(
                rule_id,
                Node::Seq(rule_body.split(" ").map(|c| c.parse().unwrap()).collect()),
            );
        }
    }
    (parts.next().unwrap(), map)
}

pub fn part1(input: &str) -> usize {
    let (messages, map) = parse(input);
    let mut match_num = 0;
    for message in messages.lines() {
        let mut chars = message.chars();
        if matches(&map, &mut chars, 0) {
            if let Some(_c) = chars.next() {
                // didn't run out of the string
                continue;
            }
            match_num += 1;
        }
    }
    match_num
}

pub fn part2(input: &str) -> usize {
    let (messages, map) = parse(input);

    let mut match_num = 0;
    for message in messages.lines() {
        let mut chars = message.chars().peekable();
        let mut matches_42 = 0;
        while matches(&map, &mut chars, 42) {
            matches_42 += 1;
            let mut innerc = chars.clone();
            for _ in 1..matches_42 {
                if matches(&map, &mut innerc, 31) {
                    let mut inner2 = innerc.clone();
                    if inner2.peek().is_none() {
                        match_num += 1;
                        break;
                    }
                }
            }
        }
    }
    match_num
}

fn match_sequence(
    map: &HashMap<usize, Node>,
    chars: &mut (impl Iterator<Item = char> + Clone),
    rules: &[usize],
) -> bool {
    let mut newchar = chars.clone();
    for &step in rules {
        if matches(map, &mut newchar, step) {
            continue;
        } else {
            return false;
        }
    }
    std::mem::swap(chars, &mut newchar);
    true
}

fn matches(
    map: &HashMap<usize, Node>,
    chars: &mut (impl Iterator<Item = char> + Clone),
    rule: usize,
) -> bool {
    match map.get(&rule).unwrap() {
        &Node::Letter(from_rule) => chars.next().unwrap_or('f') == from_rule,
        Node::Seq(steps) => match_sequence(map, chars, &steps),
        Node::Either(left, right) => {
            if match_sequence(map, chars, &left) {
                true
            } else if match_sequence(map, chars, &right) {
                true
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day19() {
        let test_input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        assert_eq!(part1(&test_input), 2);
        let test_input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        assert_eq!(part1(&test_input), 3);
    }

    #[test]
    fn test_day19_p2() {
        // this already has the rules replaced
        let test_input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        assert_eq!(part2(&test_input), 12);
    }

    #[test]
    fn day192() {}

    #[test]
    fn day19() {
        let input = get_input(2020, 19).unwrap();
        assert_eq!(part1(&input), 210);
        assert_eq!(part2(&input), 422);
    }
}
