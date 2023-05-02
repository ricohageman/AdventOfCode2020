struct Slope(usize, usize);

#[derive(Copy, Clone, Debug)]
struct Position(usize, usize);

impl Position {
    pub fn apply(self, slope: &Slope) -> Position {
        Position(self.0 + slope.0, self.1 + slope.1)
    }
}

struct Grid {
    grid: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl Grid {
    pub fn is_tree(&self, position: Position) -> bool {
        if position.1 >= self.height {
            return false;
        }

        self.grid[position.1][position.0 % self.width]
    }
}

fn determine_trees_on_slope(grid: &Grid, slope: Slope) -> usize {
    let mut current_position = Position(0, 0);
    let mut number_of_trees = 0;

    while current_position.1 < grid.height {
        current_position = current_position.apply(&slope);

        if grid.is_tree(current_position) {
            number_of_trees += 1;
        }
    }

    number_of_trees
}

fn parse_input(input: &str) -> Grid {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|element| !element.is_empty())
                .map(|element| match element {
                    "." => false,
                    "#" => true,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Grid {
        height: grid.len(),
        width: grid[0].len(),
        grid,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let slope = Slope(3, 1);

    Some(determine_trees_on_slope(&grid, slope) as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let slopes = vec![
        Slope(3, 1),
        Slope(1, 1),
        Slope(5, 1),
        Slope(7, 1),
        Slope(1, 2),
    ];

    Some(
        slopes
            .into_iter()
            .map(|slope| determine_trees_on_slope(&grid, slope))
            .product::<usize>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(336));
    }
}
