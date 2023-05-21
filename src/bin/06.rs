const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

struct AnswerSet {
    answers: [bool; ALPHABET.len()],
}

impl AnswerSet {
    pub fn yes() -> Self {
        Self {
            answers: [true; ALPHABET.len()],
        }
    }

    pub fn no() -> Self {
        Self {
            answers: [false; ALPHABET.len()],
        }
    }

    pub fn from_string(input: &str) -> Self {
        let mut answers = [false; ALPHABET.len()];

        for char in input.split("").filter(|element| !element.is_empty()) {
            answers[ALPHABET.find(char).unwrap()] = true;
        }

        Self { answers }
    }

    pub fn union(&mut self, other: &Self) {
        for index in other
            .answers
            .iter()
            .enumerate()
            .filter(|(_, element)| **element)
            .map(|(index, _)| index)
        {
            self.answers[index] = true
        }
    }

    pub fn remain_overlap(&mut self, other: &Self) {
        for index in other
            .answers
            .iter()
            .enumerate()
            .filter(|(_, element)| !**element)
            .map(|(index, _)| index)
        {
            self.answers[index] = false
        }
    }

    pub fn size(&self) -> usize {
        self.answers.iter().filter(|element| **element).count()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|line| AnswerSet::from_string(line))
                    .fold(AnswerSet::no(), |mut current, other| {
                        current.union(&other);
                        current
                    })
                    .size()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|line| AnswerSet::from_string(line))
                    .fold(AnswerSet::yes(), |mut current, other| {
                        current.remain_overlap(&other);
                        current
                    })
                    .size()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(6));
    }
}
