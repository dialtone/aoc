// day18 part 1            time:   [357.98 us 360.82 us 364.22 us]
// day18 part 2            time:   [448.65 us 456.22 us 465.16 us]

pub fn part2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        sum += solve(line, 2);
    }
    return sum;
}

fn solve(line: &str, part: u8) -> usize {
    let mut operators = vec![];
    let mut output = vec![];

    for term in line.chars() {
        match (part, term) {
            (1, '+') => {
                // each operator has the same precedence so, unless there's an
                // open paren, you just want to push to output each time.
                while operators.last() != Some(&'(') && !operators.is_empty() {
                    output.push(operators.pop().unwrap());
                }
                operators.push(term);
            }
            (1, '*') => {
                // each operator has the same precedence so, unless there's an
                // open paren, you just want to push to output each time.
                while operators.last() != Some(&'(') && !operators.is_empty() {
                    output.push(operators.pop().unwrap());
                }
                operators.push(term);
            }

            (2, '+') => {
                // + is left associative and thus needs to be pushed through
                while operators.last() == Some(&'+') {
                    output.push(operators.pop().unwrap());
                }
                operators.push(term);
            }
            (2, '*') => {
                // mul has lower priority than + so any + we push
                // through first in the output to get executed immediately
                while operators.last() == Some(&'+') {
                    output.push(operators.pop().unwrap());
                }
                // then put the *
                operators.push(term);
            }

            (_, ' ') => continue,
            (_, '(') => operators.push(term),
            (_, ')') => {
                while operators.last().unwrap() != &'(' {
                    output.push(operators.pop().unwrap());
                }
                // and take out the open paren
                operators.pop();
            }
            (_, '0'..='9') => {
                output.push(term);
            }
            _ => unreachable!(),
        }
    }

    operators.reverse();
    output.extend(operators);

    let mut stack = vec![];
    for op in output {
        match op {
            '0'..='9' => stack.push((op as u8 - b'0') as usize),
            '+' | '*' => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let res = if op == '+' { a + b } else { a * b };
                stack.push(res);
            }
            _ => unreachable!(),
        }
    }

    stack.pop().unwrap_or(0)
}

pub fn part1(s: &str) -> usize {
    let mut sum = 0;
    for line in s.lines() {
        sum += solve(line, 1);
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::get_input;

    #[test]
    fn test_day18() {
        let test_input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(part1(test_input), 71);
        let test_input = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(part1(test_input), 51);

        let test_input = "2 * 3 + (4 * 5)";
        assert_eq!(part2(test_input), 46);
    }

    #[test]
    fn day18() {
        let input = get_input(2020, 18).unwrap();
        assert_eq!(part1(&input), 654686398176);
        assert_eq!(part2(&input), 8952864356993);
    }
}
