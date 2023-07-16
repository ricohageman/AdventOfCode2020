use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BitState {
    Zero,
    One,
    Floating,
}

#[derive(Debug, Clone, Copy)]
struct BitMask([BitState; 36]);

impl BitMask {
    pub fn with_indices(&self) -> impl Iterator<Item = (usize, BitState)> + '_ {
        self.0.iter().rev().copied().enumerate()
    }

    pub fn indices_of_type(&self, t: BitState) -> impl Iterator<Item = usize> + '_ {
        self.with_indices()
            .filter(move |(_, element)| *element == t)
            .map(|(index, _)| index)
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(BitMask),
    Write { address: usize, value: usize },
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| {
        let mut data = line.split(" = ");
        let instruction = data.next().unwrap();
        let value = data.next().unwrap();

        if instruction.starts_with("mem") {
            return Instruction::Write {
                address: instruction[4..instruction.len() - 1].parse().unwrap(),
                value: value.parse().unwrap(),
            };
        }

        Instruction::Mask(BitMask(
            value
                .chars()
                .map(|char| match char {
                    '0' => BitState::Zero,
                    '1' => BitState::One,
                    'X' => BitState::Floating,
                    _ => panic!("Unexpected bit state '{char}'"),
                })
                .collect::<Vec<BitState>>()
                .try_into()
                .unwrap(),
        ))
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut instructions = parse_input(input);
    let Instruction::Mask(mask) = instructions.next().unwrap() else {
        panic!("Expected the initial instruction to be a mask.");
    };

    let mut memory: FxHashMap<usize, usize> = FxHashMap::default();
    let mut mask = mask;

    for instruction in instructions {
        match instruction {
            Instruction::Mask(updated_mask) => mask = updated_mask,
            Instruction::Write { address, value } => {
                // Binary modifications to decimal numbers based on https://stackoverflow.com/questions/74162324
                let mut mutated_value = value;

                for (index, state) in mask.with_indices() {
                    match state {
                        BitState::Zero => mutated_value = mutated_value & !(1 << index),
                        BitState::One => mutated_value = mutated_value | (1 << index),
                        BitState::Floating => {}
                    }
                }

                memory.insert(address, mutated_value);
            }
        }
    }

    Some(memory.values().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut instructions = parse_input(input);
    let Instruction::Mask(mask) = instructions.next().unwrap() else {
        panic!("Expected the initial instruction to be a mask.");
    };

    let mut memory: FxHashMap<usize, usize> = FxHashMap::default();
    let mut mask = mask;

    for instruction in instructions {
        match instruction {
            Instruction::Mask(updated_mask) => mask = updated_mask,
            Instruction::Write { address, value } => {
                // First apply the mask to the address.
                let mut mutated_address = address;

                // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
                // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
                for index in mask.indices_of_type(BitState::One) {
                    mutated_address = mutated_address | (1 << index);
                }

                // Then change the floating values in both zeros as ones.
                let addresses = mask.indices_of_type(BitState::Floating).fold(
                    vec![mutated_address],
                    |acc, index| {
                        acc.into_iter()
                            .flat_map(|address| [address | (1 << index), address & !(1 << index)])
                            .collect()
                    },
                );

                // Finally, store the value in all the memory locations.
                for address in addresses {
                    memory.insert(address, value);
                }
            }
        }
    }

    Some(memory.values().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(165));
    }

    #[test]
    fn test_part_two() {
        let input = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(part_two(&input), Some(208));
    }
}
