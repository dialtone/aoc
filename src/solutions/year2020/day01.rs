use super::*;

pub fn part1(input: &Vec<u32>) -> u32 {
    // time:   [17.688 us 17.823 us 17.961 us]
    // let m = input
    //     .iter()
    //     .tuple_combinations::<(_, _)>()
    //     .find(|x| x.0 + x.1 == 2020)
    //     .unwrap();
    // return m.0 * m.1;

    // time:   [8.2207 us 8.3451 us 8.4806 us]
    let l = input.len();
    for i in 0..l {
        for j in (i + 1)..l {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    return 0;
}

pub fn part2(input: &Vec<u32>) -> u32 {
    // time:   [1.5358 ms 1.5452 ms 1.5560 ms]
    // let m = input
    //     .iter()
    //     .tuple_combinations::<(_, _, _)>()
    //     .find(|x| x.0 + x.1 + x.2 == 2020)
    //     .unwrap();
    // return m.0 * m.1 * m.2;

    // time:   [478.39 us 485.00 us 492.58 us]
    let l = input.len();
    for i in 0..(l - 2) {
        for j in (i + 1)..(l - 1) {
            for k in (j + 1)..l {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
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
