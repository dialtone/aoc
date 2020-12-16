// day16 parse             time:   [227.19 us 231.01 us 235.46 us]
// day16 part 1            time:   [51.266 us 51.837 us 52.489 us]
// day16 part 2            time:   [432.77 us 436.99 us 441.51 us]

use super::*;
use std::ops::RangeInclusive;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::write,
};

type Input = String;
type Parsed = String;

pub fn part1(
    my_ticket: &Vec<usize>,
    tickets: &Vec<Vec<usize>>,
    rules: &BTreeMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)>,
) -> usize {
    // tickets
    //     .iter()
    //     .flatten()
    //     .filter(|field| {
    //         rules
    //             .values()
    //             .all(|(range1, range2)| !range1.contains(&field) && !range2.contains(&field))
    //     })
    //     .sum()
    let mut total = 0;
    // let mut invalid = 0;
    for ticket in tickets {
        for field in ticket {
            let mut pass = false;
            for (range1, range2) in rules.values() {
                if range1.contains(field) || range2.contains(field) {
                    pass = true;
                    break;
                }
            }
            if !pass {
                total += field;
                // invalid += 1;
            }
        }
    }
    // println!("total invalid {}", invalid);
    total
}

pub fn part2(
    my_ticket: &Vec<usize>,
    tickets: &Vec<Vec<usize>>,
    rules: &BTreeMap<&str, (RangeInclusive<usize>, RangeInclusive<usize>)>,
) -> usize {
    let mut product = 1;
    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
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
            valid_tickets.push(ticket.clone());
        }
    }

    let col_num = valid_tickets[0].len();
    // apply all rules and create sets with all the applicable columns for each field, then we'll reduce them to just
    // 1 applicable column per field
    let mut candidates = BTreeMap::new();
    for (rule_type, (range1, range2)) in rules.iter() {
        // println!("Considering field {}", rule_type);
        // apply the rules vertically in the tickets for each ticket
        for col in 0..col_num {
            // println!("Considering candidate column {}", col);

            // given column id above, apply the rule to each ticket in the list
            // and if all values pass then this is a candidate
            let mut pass = true;
            for ticket in &valid_tickets {
                let candidate = ticket[col];
                pass = range1.contains(&candidate) || range2.contains(&candidate);
                if !pass {
                    // println!(
                    //     "{} didn't satisfy this field rule {:?} or {:?}",
                    //     candidate, range1, range2
                    // );
                    break;
                }
            }
            // so if this rule passed for all values in the column
            // and wasn't already assigned to something, put it in candidates
            if pass {
                // println!(
                //     "column {} satisfied so adding to possible candidates for field {}",
                //     col, rule_type
                // );
                candidates
                    .entry(rule_type)
                    .or_insert(BTreeSet::new())
                    .insert(col);
            }
        }
    }
    // println!("---------");
    // for (&column, field) in candidates.iter() {
    //     println!("Column {} has candidates {:?}", column, field);
    // }

    // println!("---------");
    // do multiple passes to remove anything that already appears in other sets
    while !candidates.iter().all(|(_, v)| v.len() == 1) {
        let sets = candidates.clone();
        for (&k1, v) in sets {
            if v.len() == 1 {
                // println!(
                //     "Column {} is assigned field number {}",
                //     k1,
                //     v.iter().next().unwrap()
                // );
                candidates.iter_mut().for_each(|(&&k2, v1)| {
                    if k2 != k1 && v1.len() > 1 {
                        v1.remove(v.iter().next().unwrap());
                    }
                });
            }
        }
    }

    // println!("--------- FINAL ASSIGNMENTS ------");
    // for (&column, field) in candidates.iter() {
    //     println!("Column {} is assigned {:?}", column, field);
    // }

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

    //     #[test]
    //     fn test_day16() {
    //         let test_input = "class: 1-3 or 5-7
    // row: 6-11 or 33-44
    // seat: 13-40 or 45-50

    // your ticket:
    // 7,1,14

    // nearby tickets:
    // 7,3,47
    // 40,4,50
    // 55,2,20
    // 38,6,12"
    //             .to_owned();
    //         let (myt, ts, rules) = parse(&test_input);
    //         assert_eq!(part1(&myt, &ts, &rules), 71);
    //         let test_input = "departure class: 0-1 or 4-19
    // departure row: 0-5 or 8-19
    // seat: 0-13 or 16-19

    // your ticket:
    // 11,12,13

    // nearby tickets:
    // 3,9,18
    // 15,1,5
    // 5,14,9"
    //             .to_owned();
    //         let (myt, ts, rules) = parse(&test_input);
    //         assert_eq!(part2(&myt, &ts, &rules), 132);
    //     }

    #[test]
    fn day16() {
        let input = get_input(2020, 16).unwrap();
        let (myt, ts, rules) = parse(&input);
        assert_eq!(part1(&myt, &ts, &rules), 28873);
        let (myt, ts, rules) = parse(&input);
        assert_eq!(part2(&myt, &ts, &rules), 5);
    }
}
