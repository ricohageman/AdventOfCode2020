use crate::Instruction::{Accumulator, Jump, NoOperation};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum Instruction {
    Accumulator(isize),
    Jump(isize),
    NoOperation(isize),
}

enum ProgramResult {
    Terminated(isize),
    InfiniteLoop(isize),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut data = line.split(" ");
            let instruction = data.next().unwrap();
            let value: isize = data.next().unwrap().parse().unwrap();

            match instruction {
                "nop" => NoOperation(value),
                "acc" => Accumulator(value),
                "jmp" => Jump(value),
                _ => panic!("{instruction}"),
            }
        })
        .collect()
}

fn step(
    instructions: &Vec<Instruction>,
    accumulator: &mut isize,
    index: &mut isize,
    executed_instructions: &mut HashSet<isize>,
) {
    executed_instructions.insert(*index);

    match instructions[*index as usize] {
        Accumulator(amount) => {
            *accumulator += amount;
            *index += 1;
        }
        Jump(amount) => {
            *index += amount;
        }
        NoOperation(_) => {
            *index += 1;
        }
    }
}

fn simulate(
    instructions: &Vec<Instruction>,
    accumulator: isize,
    index: isize,
    executed_instructions: &mut HashSet<isize>,
) -> ProgramResult {
    let mut index = index;
    let mut accumulator = accumulator;

    loop {
        if executed_instructions.contains(&index) {
            return ProgramResult::InfiniteLoop(accumulator);
        }

        if index >= instructions.len() as isize {
            return ProgramResult::Terminated(accumulator);
        }

        step(
            instructions,
            &mut accumulator,
            &mut index,
            executed_instructions,
        );
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let instructions = parse_input(input);

    match simulate(&instructions, 0, 0, &mut HashSet::new()) {
        ProgramResult::Terminated(_) => panic!(),
        ProgramResult::InfiniteLoop(amount) => Some(amount),
    }
}

pub fn part_two(input: &str) -> Option<isize> {
    let instructions = parse_input(input);

    let mut accumulator = 0;
    let mut index = 0;
    let mut executed_instructions: HashSet<isize> = HashSet::new();

    loop {
        match instructions[index as usize] {
            Accumulator(_) => {}
            Jump(_) => {
                match simulate(
                    &instructions,
                    accumulator,
                    index + 1,
                    &mut executed_instructions.clone(),
                ) {
                    ProgramResult::Terminated(amount) => return Some(amount),
                    ProgramResult::InfiniteLoop(_) => {}
                }
            }
            NoOperation(amount) => {
                match simulate(
                    &instructions,
                    accumulator,
                    index + amount,
                    &mut executed_instructions.clone(),
                ) {
                    ProgramResult::Terminated(amount) => return Some(amount),
                    ProgramResult::InfiniteLoop(_) => {}
                }
            }
        }

        step(
            &instructions,
            &mut accumulator,
            &mut index,
            &mut executed_instructions,
        );
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
