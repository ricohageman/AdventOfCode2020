use std::cmp::{max, min, Ordering};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn solve_part_one(input: &str, preamble_size: usize) -> Option<usize> {
    let input: Vec<usize> = input.lines()
        .map(|element| element.parse::<usize>().unwrap())
        .collect();

    let mut previous_values: VecDeque<usize> = input.iter()
        .take(preamble_size)
        .copied()
        .collect();

    let mut message = input.into_iter().skip(preamble_size);

    while let Some(next) = message.next() {
        if !previous_values.iter()
            .tuple_combinations()
            .any(|(a, b)| a + b == next) {
            return Some(next);
        }

        previous_values.pop_front();
        previous_values.push_back(next);
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    solve_part_one(input, 25)
}

fn solve_part_two(input: &str, preamble_size: usize) -> Option<usize> {
    let target = solve_part_one(input, preamble_size)?;

    let input: Vec<usize> = input.lines()
        .map(|element| element.parse::<usize>().unwrap())
        .collect();

    input.iter()
        .enumerate()
        .filter_map(|(index, &start)| {
            let mut total = start;
            let mut minimum = start;
            let mut maximum = start;

            let mut remaining_values = &mut input[index + 1 ..].iter();

            while let Some(&next) = remaining_values.next() {
                total += next;
                minimum = min(minimum, next);
                maximum = max(maximum, next);

                match total.cmp(&target) {
                    Ordering::Less => continue,
                    Ordering::Equal => return Some(minimum + maximum),
                    Ordering::Greater => return None,
                }
            }

            None
        })
        .next()
}

pub fn part_two(input: &str) -> Option<usize> {
    solve_part_two(input, 25)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(solve_part_one(&input, 5), Some(127));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(solve_part_two(&input, 5), Some(62));
    }
}
