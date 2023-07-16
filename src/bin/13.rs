pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let departure_time = lines.next().unwrap().parse::<usize>().unwrap();

    lines
        .next()
        .unwrap()
        .split(",")
        .filter(|element| *element != "x")
        .map(|element| {
            let element = element.parse::<usize>().unwrap();
            let next_departure = ((departure_time / element) + 1) * element;

            (next_departure, element)
        })
        .min_by_key(|(next_departure, _)| *next_departure)
        .map(|(next_departure, bus_id)| bus_id * (next_departure - departure_time))
}

pub fn part_two(input: &str) -> Option<usize> {
    let busses = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, element)| *element != "x")
        .map(|(modulo, element)| (modulo, element.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    let increment = busses.iter().map(|(_, element)| element).min().unwrap();
    let mut current = *increment;
    let mut jump = 1;

    for (modulo, bus) in busses {
        while (current + modulo) % bus != 0 {
            current += jump;
        }

        jump *= bus;
    }

    Some(current)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(295));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(1068781));
    }

    #[test]
    fn test_part_two_examples() {
        assert_eq!(part_two("1\n17,x,13,19"), Some(3417));
        assert_eq!(part_two("1\n17,x,13,19"), Some(754018));
        assert_eq!(part_two("1\n17,x,13,19"), Some(779210));
        assert_eq!(part_two("1\n17,x,13,19"), Some(1261476));
        assert_eq!(part_two("1\n17,x,13,19"), Some(1202161486));
    }
}
