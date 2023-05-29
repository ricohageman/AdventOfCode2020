use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

type Bag = usize;

fn parse_bags(input: &str) -> HashMap<String, Bag> {
    input
        .lines()
        .enumerate()
        .map(|(index, line)| (line.split(" ").take(2).join(" "), index))
        .collect()
}

fn parse_rules(input: &str, bags: &HashMap<String, Bag>) -> HashMap<Bag, Vec<(Bag, usize)>> {
    input
        .lines()
        .map(|line| {
            let source_bag = line.split(" ").take(2).join(" ");
            let source_bag_index = bags.get(&source_bag).unwrap();

            let target_bag_indices = line
                .split("contain ")
                .skip(1)
                .next()
                .unwrap()
                .split(",")
                .filter_map(|data| {
                    if data == "no other bags." {
                        return None;
                    }

                    let mut data = data.split(" ").filter(|element| !element.is_empty());
                    let amount: usize = data.next().unwrap().parse().unwrap();

                    let target_bag = data.take(2).join(" ");
                    let target_bag_index = bags.get(&target_bag).unwrap();

                    Some((*target_bag_index, amount))
                })
                .collect();

            (*source_bag_index, target_bag_indices)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let bags = parse_bags(input);
    let rules = parse_rules(input, &bags);

    let shiny_gold_bag = bags.get("shiny gold").unwrap();
    let mut bags_to_visit = VecDeque::<Bag>::from_iter(bags.values().cloned());
    let mut contains_shiny_gold = HashMap::<Bag, bool>::new();

    while let Some(bag) = bags_to_visit.pop_front() {
        let other_bags = rules.get(&bag).unwrap();

        if other_bags.iter().any(|(bag, _)| bag == shiny_gold_bag) {
            contains_shiny_gold.insert(bag, true);
            continue;
        }

        let other_bags = other_bags
            .iter()
            .map(|(bag, _)| contains_shiny_gold.get(bag).cloned())
            .collect::<Vec<Option<bool>>>();

        if other_bags.iter().any(|element| *element == Some(true)) {
            contains_shiny_gold.insert(bag, true);
            continue;
        }

        if other_bags.iter().all(Option::is_some) {
            contains_shiny_gold.insert(bag, false);
            continue;
        }

        bags_to_visit.push_back(bag);
    }

    Some(contains_shiny_gold.values().filter(|element| **element).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let bags = parse_bags(input);
    let rules = parse_rules(input, &bags);

    let mut bags_inside = HashMap::<Bag, usize>::new();
    let mut bags_to_visit = VecDeque::<Bag>::from_iter(bags.values().cloned());

    while let Some(bag) = bags_to_visit.pop_front() {
        let result = rules.get(&bag)
            .unwrap()
            .iter()
            .map(|(other_bag, amount)| {
                bags_inside
                    .get(other_bag)
                    .map(|requirement| requirement * amount)
            })
            .fold_options(1, |acc, requirement| requirement + acc);

        if let Some(result) = result {
            bags_inside.insert(bag, result);
            continue;
        }

        bags_to_visit.push_back(bag);
    }

    Some(*bags_inside.get(bags.get("shiny gold").unwrap()).unwrap() - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(32));
    }

    #[test]
    fn test_part_two_2() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(part_two(&input), Some(126));
    }
}
