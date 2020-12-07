// day7 parse              time:   [375.69 us 378.63 us 381.82 us]
// Found 8 outliers among 100 measurements (8.00%)
//   5 (5.00%) high mild
//   3 (3.00%) high severe

// day7 part 1             time:   [22.284 us 22.432 us 22.605 us]
// Found 15 outliers among 100 measurements (15.00%)
//   5 (5.00%) high mild
//   10 (10.00%) high severe

// day7 parse_right        time:   [345.56 us 351.52 us 358.22 us]
// Found 6 outliers among 100 measurements (6.00%)
//   5 (5.00%) high mild
//   1 (1.00%) high severe

// day7 part 2             time:   [1.3245 us 1.3311 us 1.3384 us]
// Found 8 outliers among 100 measurements (8.00%)
//   5 (5.00%) high mild
//   3 (3.00%) high severe

use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: &HashMap<&str, Vec<&str>>) -> usize {
    // let mut done = HashSet::new();
    let mut gold = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back("shiny gold");
    while !queue.is_empty() {
        let bag = queue.pop_front().unwrap();

        if gold.contains(&bag) {
            continue;
        }
        if bag != "shiny gold" {
            gold.insert(bag);
        }

        if let Some(container_bags) = input.get(bag) {
            for container_bag in container_bags {
                queue.push_back(container_bag);
            }
        }
    }
    gold.len()
}

pub fn part2(input: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    recursive_walk(input, "shiny gold", 0)
}

fn recursive_walk(input: &HashMap<&str, Vec<(usize, &str)>>, bag: &str, num: usize) -> usize {
    let contained_bags = input.get(bag).unwrap();
    if contained_bags.is_empty() {
        return num;
    }

    let mut total = num;

    for (contained_bag_num, contained_bag_type) in contained_bags {
        let aggregated_contained_num = recursive_walk(input, contained_bag_type, 1);
        total += contained_bag_num * aggregated_contained_num;
    }

    total
}

pub fn parse(s: &String) -> HashMap<&str, Vec<&str>> {
    let mut inverted_rules = HashMap::new();
    for line in s.lines() {
        let (bag, contained_bags) = line
            .strip_suffix(".")
            .unwrap()
            .split_once(" contain ")
            .unwrap();
        let (trimmed_bag, _) = bag.trim().rsplit_once(" ").unwrap();
        let mut bags: Vec<(usize, &str)> = Vec::new();
        if contained_bags == "no other bags" {
            continue;
        }
        for contained_bag in contained_bags.split(",") {
            let (proper_bag, _) = contained_bag.trim().rsplit_once(" ").unwrap();
            let (bag_number, bag_type) = proper_bag.split_once(" ").unwrap();
            let entry = inverted_rules.entry(bag_type).or_insert(Vec::new());
            entry.push(trimmed_bag);
        }
    }
    inverted_rules
}

pub fn parse_right(s: &String) -> HashMap<&str, Vec<(usize, &str)>> {
    let mut rules = HashMap::new();
    for line in s.lines() {
        let (bag, contained_bags) = line
            .strip_suffix(".")
            .unwrap()
            .split_once(" contain ")
            .unwrap();
        let (trimmed_bag, _) = bag.trim().rsplit_once(" ").unwrap();
        let mut bags: Vec<(usize, &str)> = Vec::new();
        if contained_bags == "no other bags" {
            rules.insert(trimmed_bag, Vec::new());
            continue;
        }
        for contained_bag in contained_bags.split(",") {
            let (proper_bag, _) = contained_bag.trim().rsplit_once(" ").unwrap();
            let (bag_number, bag_type) = proper_bag.split_once(" ").unwrap();
            bags.push((bag_number.trim().parse().unwrap(), bag_type))
        }
        rules.insert(trimmed_bag, bags);
    }
    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day07() {
        let test_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .to_owned();
        assert_eq!(part1(&parse(&test_input)), 4);
        assert_eq!(part2(&parse_right(&test_input)), 32);
    }

    #[test]
    fn day07() {
        let input = get_input(2020, 7).unwrap();
        assert_eq!(part1(&parse(&input)), 211);
        assert_eq!(part2(&parse_right(&input)), 12414);
    }
}
