// too many hints in this one...
// day23 part 1            time:   [2.4305 us 2.4627 us 2.5082 us]
// day23 part 1            time:   [21.119 us 21.521 us 22.027 us] alt way
// day23 part 2            time:   [224.32 ms 227.20 ms 230.39 ms]
use super::*;

// alt way to do it.
// pub fn part1(input: &str) -> String {
//     let mut cups = input.chars().map(|x| x as u8 - b'0').collect::<Vec<u8>>();
//     let min = *cups.iter().min().unwrap();
//     let max = *cups.iter().max().unwrap();

//     for _ in 0..100 {
//         let mut destination_label = cups[0] - 1;
//         if destination_label < min {
//             destination_label = max;
//         }

//         // find correct destination_label
//         while cups
//             .iter()
//             .enumerate()
//             .find(|&(i, cup)| *cup == destination_label && i >= 4)
//             == None
//         {
//             destination_label -= 1;
//             if destination_label < min {
//                 destination_label = max;
//             }
//         }

//         // find position of the label now
//         let position = cups
//             .iter()
//             .position(|cup| *cup == destination_label)
//             .unwrap();

//         // basically put the cups we cared about behind the position
//         cups[1..position + 1].rotate_left(3);
//         // next phase we want the current cup to be still 0
//         cups.rotate_left(1);
//     }

//     while cups[0] != 1 {
//         cups.rotate_left(1);
//     }
//     cups[1..].iter().map(|x| x.to_string()).join("")
// }

pub fn part1(input: &str) -> String {
    let cups = input
        .trim()
        .chars()
        .map(|x| (x as u8 - b'0') as usize)
        .collect::<Vec<usize>>();

    let size = cups.len();
    let min = 1;
    let max = 9;

    let mut nextcup = vec![0; 10];
    for (i, &cup) in cups.iter().enumerate() {
        nextcup[cup] = cups[(i + 1) % size];
    }

    let mut cup = cups[0];
    for _ in 0..100 {
        // grab first 3 after current cup
        let rem1 = nextcup[cup];
        let rem2 = nextcup[rem1];
        let rem3 = nextcup[rem2];

        // reconnect the current cup to this
        let next = nextcup[rem3];
        nextcup[cup] = next;

        // find correct destination_label
        let mut destination_label = cup - 1;
        if destination_label < min {
            destination_label = max;
        }

        while destination_label == rem1 || destination_label == rem2 || destination_label == rem3 {
            destination_label = destination_label - 1;
            if destination_label < min {
                destination_label = max;
            }
        }

        let next = nextcup[destination_label];
        nextcup[destination_label] = rem1;
        nextcup[rem3] = next;

        cup = nextcup[cup];
    }

    let mut acc = vec![];
    let mut prev = 1;

    for _ in 0..8 {
        prev = nextcup[prev];
        acc.push(prev.to_string());
    }
    acc.iter().join("")
}

pub fn part2(input: &str) -> usize {
    let cups = input
        .trim()
        .chars()
        .map(|x| (x as u8 - b'0') as usize)
        .collect::<Vec<usize>>();

    let cups = cups
        .into_iter()
        .chain(10..=1_000_000)
        .collect::<Vec<usize>>();

    let size = cups.len();
    let min = 1;
    let max = 1_000_000;

    let mut nextcup = vec![0; 1_000_001];
    for (i, &cup) in cups.iter().enumerate() {
        nextcup[cup] = cups[(i + 1) % size];
    }

    let mut cup = cups[0];
    for _ in 0..10_000_000 {
        // grab first 3 after current cup
        let rem1 = nextcup[cup];
        let rem2 = nextcup[rem1];
        let rem3 = nextcup[rem2];

        // reconnect the current cup to this
        let next = nextcup[rem3];
        nextcup[cup] = next;

        // find correct destination_label
        let mut destination_label = cup - 1;
        if destination_label < min {
            destination_label = max;
        }

        while destination_label == rem1 || destination_label == rem2 || destination_label == rem3 {
            destination_label = destination_label - 1;
            if destination_label < min {
                destination_label = max;
            }
        }

        let next = nextcup[destination_label];
        nextcup[destination_label] = rem1;
        nextcup[rem3] = next;

        cup = nextcup[cup];
    }
    let a = nextcup[1];
    let b = nextcup[a];

    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day23() {
        let test_input = "389125467";
        assert_eq!(part1(test_input), "67384529".to_string());
        assert_eq!(part2(test_input), 149245887792);
    }

    #[test]
    fn day23() {
        let input = "784235916";
        assert_eq!(part1(&input), "53248976".to_string());
        assert_eq!(part2(&input), 418819514477);
    }
}
