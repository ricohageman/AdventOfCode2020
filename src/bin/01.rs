use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let numbers: HashSet<u32> = input
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    numbers.iter()
        .filter_map(|&n| {
            let counter_part = 2020 - n;
            numbers.contains(&counter_part).then(|| n * counter_part)
        })
        .next()
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: HashSet<u32> = input
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    numbers
        .iter()
        .combinations(3)
        .map(|values| (values[0], values[1], values[2]))
        .filter_map(|(a, b, c)| {
            if a + b + c != 2020 {
                return None;
            }

            Some(a * b * c)
        })
        .next()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(514579));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(241861950));
    }
}
