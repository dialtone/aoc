use super::*;

pub fn part1(input: &Vec<u32>) -> u32 {
    let l = input.len();
    let mut start = 0;
    while start < l {
        for i in start..l {
            for j in (i + 1)..l {
                if input[i] + input[j] == 2020 {
                    return input[i] * input[j];
                }
            }
        }
        start = start + 1;
    }
    return 0;
}

pub fn part2(input: &Vec<u32>) -> u32 {
    let l = input.len();
    let mut start = 0;
    while start < l {
        for i in start..(l - 2) {
            for j in (i + 1)..(l - 1) {
                for k in (j + 1)..l {
                    if input[i] + input[j] + input[k] == 2020 {
                        return input[i] * input[j] * input[k];
                    }
                }
            }
        }
        start = start + 1;
    }
    return 0;
}

pub fn parse(s: String) -> Vec<u32> {
    s.trim()
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_test() {
        let expense_report = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&expense_report), 514579);
        assert_eq!(part2(&expense_report), 241861950);
    }

    #[test]
    fn day01() {
        let input = get_input(2020, 1).unwrap();
        let expense_report = parse(input);
        assert_eq!(part1(&expense_report), 876459);
        assert_eq!(part2(&expense_report), 116168640);
    }
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let expenses = parse(get_input(2020, 1).unwrap());
        b.iter(|| (part1(&expenses), part2(&expenses)));
    }
}
