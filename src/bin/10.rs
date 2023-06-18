use itertools::Itertools;
use std::iter;

pub fn part_one(input: &str) -> Option<usize> {
    let adapters: Vec<_> = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .sorted()
        .collect();

    let built_in_adapter = *adapters.iter().last().unwrap() + 3;

    let (_, one, three) = adapters
        .into_iter()
        .chain(iter::once(built_in_adapter))
        .fold(
            (0, 0, 0),
            |(joltage, one_jump, three_jump), adapter_rating| {
                let difference = adapter_rating - joltage;

                match difference {
                    0 => panic!(),
                    1 => (adapter_rating, one_jump + 1, three_jump),
                    2 => (adapter_rating, one_jump, three_jump),
                    3 => (adapter_rating, one_jump, three_jump + 1),
                    _ => panic!(),
                }
            },
        );

    Some(one * three)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut adapters: Vec<_> = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .sorted()
        .collect();

    let built_in_adapter = *adapters.iter().last().unwrap() + 3;
    adapters.push(built_in_adapter);

    let mut dynamic_program: Vec<usize> = vec![0; built_in_adapter as usize + 1];
    dynamic_program[0] = 1;

    for adapter in adapters {
        dynamic_program[adapter as usize] = match adapter {
            0 => panic!(),
            1 => dynamic_program[0],
            2 => dynamic_program[0] + dynamic_program[1],
            _ => {
                dynamic_program[adapter as usize - 1]
                    + dynamic_program[adapter as usize - 2]
                    + dynamic_program[adapter as usize - 3]
            }
        };
    }

    dynamic_program.last().copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(35));
    }
}
