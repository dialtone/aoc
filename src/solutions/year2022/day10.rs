//
pub fn cpu(input: &[u8], mut fun: impl FnMut(i32, i32)) {
    let mut program = input
        .split(|c| c == &b'\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            if line.len() == 4 {
                (line, 0)
            } else {
                (&line[..4], atoi::atoi::<i32>(&line[5..]).unwrap())
            }
        });

    let mut x: i32 = 1;

    let mut cmd = None;
    for cycle in 1.. {
        fun(cycle, x);
        if let Some((_, n)) = cmd {
            x += n;
            cmd = None;
        } else {
            cmd = match program.next() {
                None => break,
                Some((b"noop", _)) => None,
                Some(cmd) => Some(cmd),
            }
        }
    }
}

// year 22 day10 part 1    time:   [1.3668 µs 1.3719 µs 1.3781 µs]
pub fn part1(input: &[u8]) -> i32 {
    let mut observations = 0;
    cpu(input, |cycle, x| {
        if cycle == 20 + 40 * (cycle / 40) {
            observations += cycle * x;
        }
    });

    observations
}

// year 22 day10 part 2    time:   [1.5127 µs 1.5151 µs 1.5182 µs]
pub fn part2(input: &[u8]) -> usize {
    let mut crt: Vec<char> = vec!['.'; 240];
    cpu(input, |cycle, x| {
        let x_pos = (cycle - 1) % 40;
        if x_pos == x || x_pos == x + 1 || x_pos == x - 1 {
            let cursor = 40 * ((cycle - 1) / 40) + x_pos;
            crt[cursor as usize] = '#';
        }
    });

    // for row in 0..6 {
    //     // FZBPBFZF
    //     println!(
    //         "{}",
    //         crt[row * 40..row * 40 + 40].iter().collect::<String>()
    //     );
    // }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::*;

    #[test]
    fn day10_test() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(part1(input.as_bytes()), 13140);
        assert_eq!(part2(input.as_bytes()), 0);
    }

    #[test]
    fn day10() {
        let input = get_input(2022, 10).unwrap();
        assert_eq!(part1(input.as_bytes()), 14720);
        assert_eq!(part2(input.as_bytes()), 0);
    }
}
