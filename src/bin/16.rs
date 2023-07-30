use arrayvec::ArrayVec;
use itertools::Itertools;
use std::collections::VecDeque;

struct Ticket();

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

    let mut complete_solution: Option<ArrayVec<usize, SIZE>> = None;
    let mut queue: VecDeque<ArrayVec<usize, SIZE>> =
        VecDeque::from_iter(valid_field_indices_per_index[0].iter().map(|index| {
            let mut vec = ArrayVec::<usize, SIZE>::new();
            vec.push(*index);
            vec
        }));

    while let Some(assignment) = queue.pop_front() {
        if assignment.is_full() {
            complete_solution = Some(assignment);
            break;
        }

        valid_field_indices_per_index[assignment.len()]
            .iter()
            .filter(|index| !assignment.contains(index))
            .for_each(|index| {
                let mut assignment = assignment.clone();
                assignment.push(*index);
                queue.push_front(assignment);
            });
    }

    complete_solution.map(|solution| {
        solution
            .into_iter()
            .enumerate()
            .sorted_by_key(|(_, field_index)| *field_index)
            .take(6)
            .map(|(ticket_index, _)| ticket_index)
            .map(|index| ticket[index])
            .product()
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    let result = part_two::<20>(input);
    println!("{:?}", result);
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
