use rustc_hash::FxHashMap;

pub struct History {
    history: [Option<usize>; 2],
}

impl History {
    pub fn new(iteration: usize) -> Self {
        Self {
            history: [None, Some(iteration)],
        }
    }

    pub fn next(&mut self, iteration: usize) {
        self.history[0] = self.history[1];
        self.history[1] = Some(iteration);
    }
}

fn solve(input: &str, iterations: usize) -> usize {
    let mut history: FxHashMap<usize, History> = input
        .split(",")
        .map(|input| input.parse::<usize>().unwrap())
        .enumerate()
        .map(|(iteration, number)| (number, History::new(iteration)))
        .collect();

    let mut number: usize = input.split(",").last().unwrap().parse().unwrap();

    for iteration in history.len()..iterations {
        number = match history[&number].history {
            [None, Some(_)] => 0,
            [Some(a), Some(b)] => b - a,
            _ => panic!(),
        };

        history
            .entry(number)
            .and_modify(|current| current.next(iteration))
            .or_insert(History::new(iteration));
    }

    number
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 2020))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 30000000))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&"0,3,6"), Some(436));
        assert_eq!(part_one(&"1,3,2"), Some(1));
        assert_eq!(part_one(&"2,1,3"), Some(10));
        assert_eq!(part_one(&"1,2,3"), Some(27));
        assert_eq!(part_one(&"2,3,1"), Some(78));
        assert_eq!(part_one(&"3,2,1"), Some(438));
        assert_eq!(part_one(&"3,1,2"), Some(1836));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&"0,3,6"), Some(175594));
        assert_eq!(part_two(&"1,3,2"), Some(2578));
        assert_eq!(part_two(&"2,1,3"), Some(3544142));
        assert_eq!(part_two(&"1,2,3"), Some(261214));
        assert_eq!(part_two(&"2,3,1"), Some(6895259));
        assert_eq!(part_two(&"3,2,1"), Some(18));
        assert_eq!(part_two(&"3,1,2"), Some(362));
    }
}
