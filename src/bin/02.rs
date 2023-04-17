use std::ops::Range;

#[derive(Default)]
struct PasswordPolicy {
    pub password: String,
    pub character: String,
    pub first: usize,
    pub second: usize,
}

fn parse_input(input: &str) -> Vec<PasswordPolicy> {
    input
        .lines()
        .map(|line| {
            let mut data = line.split(": ");
            let requirement = data.next().unwrap();
            let password = data.next().unwrap();

            let mut requirement_data = requirement.split(" ");
            let mut occurences_data = requirement_data.next().unwrap().split("-");
            let required_character = requirement_data.next().unwrap();

            let first = occurences_data.next().unwrap().parse().unwrap();
            let second = occurences_data.next().unwrap().parse().unwrap();

            PasswordPolicy {
                password: password.to_string(),
                character: required_character.to_string(),
                first,
                second,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .filter(|password_policy| {
                let occurrences = password_policy
                    .password
                    .matches(&password_policy.character)
                    .count();

                (password_policy.first..=password_policy.second).contains(&occurrences)
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .filter(|password_policy| {
                let first_character_matches = password_policy.password.as_bytes()
                    [password_policy.first - 1]
                    == password_policy.character.as_bytes()[0];
                let second_character_matches = password_policy.password.as_bytes()
                    [password_policy.second - 1]
                    == password_policy.character.as_bytes()[0];

                match (first_character_matches, second_character_matches) {
                    (false, false) => false,
                    (true, true) => false,
                    (_, _) => true,
                }
            })
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(1));
    }
}
