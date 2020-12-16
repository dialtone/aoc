// day16 parse             time:   [235.90 us 241.36 us 247.52 us]
// day16 part 1            time:   [52.535 us 53.690 us 55.173 us]
// day16 part 2            time:   [266.19 us 270.68 us 275.73 us]

use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::RangeInclusive;

type Input = String;

pub fn part1(
    _my_ticket: &Vec<usize>,
    tickets: &Vec<Vec<usize>>,
    rules: &BTreeMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)>,
) -> usize {
    tickets
        .iter()
        .flatten()
        .filter(|field| {
            rules
                .values()
                .all(|r| !r.0.contains(&field) && !r.1.contains(&field))
        })
        .sum()
}

pub fn part2(
    my_ticket: &Vec<usize>,
    tickets: &Vec<Vec<usize>>,
    rules: &BTreeMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)>,
) -> usize {
    let mut product = 1;
    // Made this Vec<&[size]> instead of Vec<Vec<usize>> to avoid borrow checker anger
    let mut valid_tickets: Vec<&[usize]> = Vec::new();
    for ticket in tickets {
        let mut pass = false;
        for field in ticket {
            pass = false;
            for (range1, range2) in rules.values() {
                if range1.contains(&field) || range2.contains(&field) {
                    pass = true;
                    break;
                }
            }
            if !pass {
                break;
            }
        }
        if pass {
            valid_tickets.push(ticket);
        }
    }

    let col_num = valid_tickets[0].len();
    // apply all rules and create sets with all the applicable columns for each field, then we'll reduce them to just
    // 1 applicable column per field
    let mut candidates = BTreeMap::new();
    for (rule_type, (range1, range2)) in rules.iter() {
        // apply the rules vertically in the tickets for each ticket
        for col in 0..col_num {
            // given column id above, apply the rule to each ticket in the list
            // and if all values pass then this is a candidate
            let mut pass = true;
            for ticket in &valid_tickets {
                let candidate = ticket[col];
                pass = range1.contains(&candidate) || range2.contains(&candidate);
                if !pass {
                    break;
                }
            }

            // so if this rule passed for all values in the column
            // and wasn't already assigned to something, put it in candidates
            if pass {
                candidates
                    .entry(rule_type)
                    .or_insert(BTreeSet::new())
                    .insert(col);
            }
        }
    }

    // do multiple passes to remove anything that already appears in other sets
    while !candidates.iter().any(|(_, v)| v.len() > 1) {
        // this really punishes performance
        let sets = candidates.clone();
        for (&k1, v1) in sets {
            if v1.len() == 1 {
                candidates.iter_mut().for_each(|(&&k2, v2)| {
                    if k2 != k1 && v2.len() > 1 {
                        v2.remove(v1.iter().next().unwrap());
                    }
                });
            }
        }
    }

    for (&column, field) in candidates.iter() {
        if column.starts_with("departure") {
            product *= my_ticket[*field.iter().next().unwrap()];
        }
    }
    product
}

pub fn parse(
    s: &Input,
) -> (
    Vec<usize>,
    Vec<Vec<usize>>,
    BTreeMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)>,
) {
    let mut stage = 0;
    let mut tickets = Vec::new();
    let mut my_ticket = Vec::new();
    let mut rules = BTreeMap::new();
    for l in s.lines() {
        if l == "your ticket:" {
            stage += 1;
            continue;
        } else if l == "" {
            continue;
        } else if l == "nearby tickets:" {
            continue;
        }

        if stage == 0 {
            let (rule_name, low1_range, high1_range, low2_range, high2_range): (
                &str,
                usize,
                usize,
                usize,
                usize,
            ) = scan!("{}: {}-{} or {}-{}" <- l).unwrap();
            rules.insert(
                rule_name,
                (low1_range..=high1_range, low2_range..=high2_range),
            );
        } else if stage == 1 {
            my_ticket = l.split(",").map(|x| x.parse().unwrap()).collect();
            stage += 1
        } else {
            let ticket = l.split(",").map(|x| x.parse().unwrap()).collect();
            tickets.push(ticket);
        }
    }
    (my_ticket, tickets, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16() {
        let test_input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .to_owned();
        let (myt, ts, rules) = parse(&test_input);
        assert_eq!(part1(&myt, &ts, &rules), 71);
        let test_input = "departure class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            .to_owned();
        let (myt, ts, rules) = parse(&test_input);
        assert_eq!(part2(&myt, &ts, &rules), 132);
    }

    #[test]
    fn day16() {
        let input = get_input(2020, 16).unwrap();
        let (myt, ts, rules) = parse(&input);
        assert_eq!(part1(&myt, &ts, &rules), 28873);
        let (myt, ts, rules) = parse(&input);
        assert_eq!(part2(&myt, &ts, &rules), 56587952167);
    }
}
