use arrayvec::ArrayVec;
use itertools::Itertools;
use rustc_hash::FxHashSet;

fn parse_input(input: &str) -> (Vec<Vec<(usize, usize)>>, Vec<usize>, Vec<Vec<usize>>) {
    let mut input = input.split("\n\n");

    let fields = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(": ")
                .skip(1)
                .next()
                .unwrap()
                .split(" or ")
                .map(|range| {
                    let mut data = range.split("-");
                    (
                        data.next().unwrap().parse::<usize>().unwrap(),
                        data.next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let ticket = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|element| element.parse::<usize>().unwrap())
        .collect();

    let nearby_tickets = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|element| element.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (fields, ticket, nearby_tickets)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (fields, _, nearby_tickets) = parse_input(input);

    Some(
        nearby_tickets
            .into_iter()
            .filter_map(|ticket| {
                ticket
                    .into_iter()
                    .filter(|element| {
                        fields.iter().all(|field| {
                            field
                                .iter()
                                .all(|(start, end)| element < start || element > end)
                        })
                    })
                    .next()
            })
            .sum(),
    )
}

pub fn solve_part_two(input: &str) -> Option<usize> {
    part_two::<20>(input)
}

pub fn part_two<const SIZE: usize>(input: &str) -> Option<usize> {
    let (fields, ticket, nearby_tickets) = parse_input(input);
    assert_eq!(fields.len(), SIZE);

    let valid_tickets: Vec<Vec<usize>> = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket.iter().all(|number| {
                fields.iter().any(|field| {
                    field
                        .iter()
                        .any(|(start, end)| start <= number && number <= end)
                })
            })
        })
        .collect();

    let valid_field_indices_per_index: ArrayVec<Vec<usize>, SIZE> = (0..SIZE)
        .map(|index| {
            fields
                .iter()
                .enumerate()
                .filter(|(_, field)| {
                    valid_tickets.iter().all(|ticket| {
                        let number = &ticket[index];

                        field
                            .iter()
                            .any(|(start, end)| start <= number && number <= end)
                    })
                })
                .map(|(field_index, _)| field_index)
                .collect()
        })
        .collect();


    let mut complete_solution: ArrayVec<Option<usize>, SIZE> = ArrayVec::from([None; SIZE]);

    let mut used_rule_indices: FxHashSet<usize> = FxHashSet::with_capacity_and_hasher(SIZE, Default::default());
    let mut known_field_indices: FxHashSet<usize> = FxHashSet::with_capacity_and_hasher(SIZE, Default::default());

    while complete_solution.iter().any(|index| index.is_none()) {
        let (field_index, rule_index) = valid_field_indices_per_index
            .iter()
            .enumerate()
            .filter(|(field_index, _)| !known_field_indices.contains(field_index))
            .filter_map(|(field_index, valid_rule_indices)| {
                let mut valid_unused_rules = valid_rule_indices.iter()
                    .filter(|rule_index| !used_rule_indices.contains(rule_index))
                    .peekable();
                let potential_single_unused_rule = valid_unused_rules.next().unwrap();

                if valid_unused_rules.peek().is_some() {
                    return None;
                }

                Some((field_index, *potential_single_unused_rule))
            })
            .next()
            .unwrap();

        complete_solution[field_index] = Some(rule_index);
        used_rule_indices.insert(rule_index);
        known_field_indices.insert(field_index);
    }

    Some(
        complete_solution
            .into_iter()
            .map(|index| index.unwrap())
            .enumerate()
            .sorted_by_key(|(_, field_index)| *field_index)
            .take(6)
            .map(|(ticket_index, _)| ticket_index)
            .map(|index| ticket[index])
            .product()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, solve_part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(71));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two::<3>(&input), Some(98));
    }
}
