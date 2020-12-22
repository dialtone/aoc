// day22 part 1            time:   [4.7868 us 4.8425 us 4.9031 us]
// day22 part 2            time:   [428.49 ms 431.63 ms 434.89 ms]
// with a single check if you want to cheat?
// day22 part 2            time:   [123.03 ms 124.29 ms 125.72 ms]
// the right check instead runs in 700ms
// day22 part 2            time:   [695.97 ms 699.92 ms 704.00 ms]

use super::*;

use std::collections::*;

pub fn part1(input: &str) -> usize {
    let mut decks = vec![];
    for player_deck in input.split("\n\n") {
        let mut deck = player_deck.lines();
        deck.next().unwrap();
        decks.push(
            deck.map(|card| card.parse().unwrap())
                .collect::<VecDeque<usize>>(),
        );
    }
    let mut d = decks.into_iter();
    let mut deck1 = d.next().unwrap();
    let mut deck2 = d.next().unwrap();

    while !deck1.is_empty() && !deck2.is_empty() {
        let player1_card = deck1.pop_front().unwrap();
        let player2_card = deck2.pop_front().unwrap();

        if player1_card > player2_card {
            deck1.push_back(player1_card);
            deck1.push_back(player2_card);
        } else {
            deck2.push_back(player2_card);
            deck2.push_back(player1_card);
        }
    }
    let winning_deck = if deck1.is_empty() { deck2 } else { deck1 };

    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut decks = vec![];
    for player_deck in input.split("\n\n") {
        let mut deck = player_deck.lines();
        deck.next().unwrap();
        decks.push(
            deck.map(|card| card.parse().unwrap())
                .collect::<VecDeque<usize>>(),
        );
    }
    let mut d = decks.into_iter();
    let deck1 = d.next().unwrap();
    let deck2 = d.next().unwrap();

    let (winner, deck1, deck2) = play_game(deck1, deck2);

    let winning_deck = if winner == 1 { deck1 } else { deck2 };

    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum()
}

fn play_game(
    mut deck1: VecDeque<usize>,
    mut deck2: VecDeque<usize>,
) -> (usize, VecDeque<usize>, VecDeque<usize>) {
    // let mut seen1 = HashSet::new();
    // let mut seen2 = HashSet::new();
    let mut seen = HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        // if seen1.contains(&deck1) && seen2.contains(&deck2) {
        //     return (1, deck1, deck2);
        // }
        // seen1.insert(deck1.clone());
        // seen2.insert(deck2.clone());

        let decks = (deck1.clone(), deck2.clone());
        if seen.contains(&decks) {
            return (1, deck1, deck2);
        }
        seen.insert(decks);

        let player1_card = deck1.pop_front().unwrap();
        let player2_card = deck2.pop_front().unwrap();

        if deck1.len() >= player1_card && deck2.len() >= player2_card {
            let subdeck1 = deck1
                .iter()
                .copied()
                .take(player1_card)
                .collect::<VecDeque<usize>>();
            let subdeck2 = deck2
                .iter()
                .copied()
                .take(player2_card)
                .collect::<VecDeque<usize>>();
            let (winner, _, _) = play_game(subdeck1, subdeck2);

            if winner == 1 {
                deck1.push_back(player1_card);
                deck1.push_back(player2_card);
            } else {
                deck2.push_back(player2_card);
                deck2.push_back(player1_card);
            }
            continue;
        }

        if player1_card > player2_card {
            deck1.push_back(player1_card);
            deck1.push_back(player2_card);
        } else {
            deck2.push_back(player2_card);
            deck2.push_back(player1_card);
        }
    }

    let winner = if deck1.is_empty() { 2 } else { 1 };
    (winner, deck1, deck2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day22() {
        let test_input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(part1(&test_input), 306);
        assert_eq!(part2(&test_input), 291);
    }

    #[test]
    fn day22() {
        let input = get_input(2020, 22).unwrap();
        assert_eq!(part1(&input), 30138);
        assert_eq!(part2(&input), 31587);
    }
}
