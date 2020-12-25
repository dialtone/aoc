// day25 part 1            time:   [95.715 ms 97.089 ms 98.552 ms]
use super::*;

pub fn part1(input: &str) -> usize {
    let mut pubkeys = input.lines();

    let pk1 = pubkeys.next().unwrap().parse::<usize>().unwrap();
    let pk2 = pubkeys.next().unwrap().parse::<usize>().unwrap();

    let loop_size1 = get_loop_size_from_pubkey(pk1);
    //let _loop_size2 = get_loop_size_from_pubkey(pk2);

    get_encryption(loop_size1, pk2)
}

fn get_encryption(loop_size: usize, pubkey: usize) -> usize {
    let mut value = 1;

    for _ in 0..loop_size {
        value *= pubkey;
        value = value % 20201227;
    }

    value
}

fn get_loop_size_from_pubkey(pubkey: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;

    while value != pubkey {
        loop_size += 1;

        value *= 7;
        value = value % 20201227;
    }

    loop_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day25() {
        let test_input = "5764801
17807724";
        assert_eq!(part1(test_input), 14897079);
    }

    #[test]
    fn day25() {
        let input = get_input(2020, 25).unwrap();
        assert_eq!(part1(&input), 3217885);
    }
}
