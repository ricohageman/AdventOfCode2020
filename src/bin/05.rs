use std::collections::HashSet;

fn binary_search(min: usize, max: usize, left: char, right: char, sequence: Vec<char>) -> usize {
    let mut min = min;
    let mut max = max;

    for char in sequence {
        if char == left {
            max = (min + max) / 2;
        } else if char == right {
            min = (min + max + 1) / 2;
        } else {
            panic!("{}", char);
        }
    }

    min
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BoardingPass {
    row: usize,
    column: usize,
    seat_id: usize,
}

impl BoardingPass {
    pub fn new(input: &str) -> Self {
        let chars = input.chars();

        let row = binary_search(0, 127, 'F', 'B', chars.clone().take(7).collect());
        let column = binary_search(0, 7, 'L', 'R', chars.clone().skip(7).take(3).collect());

        Self {
            row: row.clone(),
            column: column.clone(),
            seat_id: row * 8 + column,
        }
    }
}

fn parse_input(input: &str) -> Vec<BoardingPass> {
    input.lines().map(|line| BoardingPass::new(line)).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    parse_input(input)
        .iter()
        .map(|boarding_pass| boarding_pass.seat_id)
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let seat_ids: HashSet<usize> = parse_input(input)
        .iter()
        .map(|boarding_pass| boarding_pass.seat_id)
        .collect();

    let highest_seat_id = *seat_ids.iter().max().unwrap();

    (1..highest_seat_id)
        .filter(|seat_id| !seat_ids.contains(seat_id))
        .filter(|seat_id| seat_ids.contains(&(seat_id - 1)) && seat_ids.contains(&(seat_id + 1)))
        .next()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_boarding_passes() {
        assert_eq!(
            BoardingPass::new("FBFBBFFRLR"),
            BoardingPass {
                row: 44,
                column: 5,
                seat_id: 357,
            }
        );

        assert_eq!(
            BoardingPass::new("BFFFBBFRRR"),
            BoardingPass {
                row: 70,
                column: 7,
                seat_id: 567,
            }
        );

        assert_eq!(
            BoardingPass::new("FFFBBBFRRR"),
            BoardingPass {
                row: 14,
                column: 7,
                seat_id: 119,
            }
        );

        assert_eq!(
            BoardingPass::new("BBFFBBFRLL"),
            BoardingPass {
                row: 102,
                column: 4,
                seat_id: 820,
            }
        );
    }
}
