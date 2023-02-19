use petgraph::{graphmap::DiGraphMap, EdgeDirection::*};
use std::collections::{HashSet, VecDeque};

pub fn part1(input: &DiGraphMap<&str, usize>) -> usize {
    let mut done = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back("shiny gold");
    while let Some(bag) = queue.pop_front() {
        if done.contains(&bag) {
            continue;
        }
        if bag != "shiny gold" {
            done.insert(bag);
        }

        for neigh in input.neighbors_directed(bag, Incoming) {
            queue.push_back(neigh);
        }
    }
    done.len()
}

pub fn part2(input: &DiGraphMap<&str, usize>) -> usize {
    let mut queue = VecDeque::new();

    let mut total = 0;
    queue.push_back(("shiny gold", 1));
    while let Some((bag, num)) = queue.pop_front() {
        total += num;
        for (_, neigh, e) in input.edges(bag) {
            queue.push_back((neigh, num * e));
        }
    }
    total - 1 // gold doesn't count
}

pub fn parse(s: &String) -> DiGraphMap<&str, usize> {
    let mut rules = DiGraphMap::new();
    for line in s.lines() {
        let (bag, contained_bags) = line
            .strip_suffix(".")
            .unwrap()
            .split_once(" contain ")
            .unwrap();
        let (trimmed_bag, _) = bag.trim().rsplit_once(" ").unwrap();
        if contained_bags == "no other bags" {
            continue;
        }
        for contained_bag in contained_bags.split(",") {
            let (proper_bag, _) = contained_bag.trim().rsplit_once(" ").unwrap();
            let (bag_number, bag_type) = proper_bag.split_once(" ").unwrap();
            rules.add_edge(trimmed_bag, bag_type, bag_number.parse().unwrap());
        }
    }
    rules
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::get_input;

    #[test]
    fn test_day07_graph() {
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
        assert_eq!(part2(&parse(&test_input)), 32);
    }

    #[test]
    fn day07_graph() {
        let input = get_input(2020, 7).unwrap();
        assert_eq!(part1(&parse(&input)), 211);
        assert_eq!(part2(&parse(&input)), 12414);
    }
}
