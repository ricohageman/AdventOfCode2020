#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Right,
    Left,
    Around,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match (self, turn) {
            // North
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::North, Turn::Around) => Direction::South,
            (Direction::North, Turn::Right) => Direction::East,

            // East
            (Direction::East, Turn::Left) => Direction::North,
            (Direction::East, Turn::Around) => Direction::West,
            (Direction::East, Turn::Right) => Direction::South,

            // South
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::South, Turn::Around) => Direction::North,
            (Direction::South, Turn::Right) => Direction::West,

            // West
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::West, Turn::Around) => Direction::East,
            (Direction::West, Turn::Right) => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(Direction, isize),
    Forward(isize),
    Turn(Turn),
}

#[derive(Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn transition(self, direction: Direction, amount: isize) -> Self {
        let (x, y) = match direction {
            Direction::North => (self.x, self.y + amount),
            Direction::East => (self.x + amount, self.y),
            Direction::South => (self.x, self.y - amount),
            Direction::West => (self.x - amount, self.y),
        };

        Self { x, y }
    }

    pub fn move_towards(self, waypoint: RelativePosition, amount: isize) -> Self {
        Self {
            x: self.x + waypoint.x * amount,
            y: self.y + waypoint.y * amount,
        }
    }
}

#[derive(Clone, Copy)]
struct RelativePosition {
    x: isize,
    y: isize,
}

impl RelativePosition {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn transition(self, direction: Direction, amount: isize) -> Self {
        let position = Position::new(self.x, self.y).transition(direction, amount);
        Self::new(position.x, position.y)
    }

    pub fn turn(self, turn: Turn) -> Self {
        let (x, y) = match turn {
            Turn::Right => (self.y, -self.x),
            Turn::Left => (-self.y, self.x),
            Turn::Around => (-self.x, -self.y),
        };

        Self::new(x, y)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Action> + '_ {
    input.lines().map(|line| {
        let mut chars = line.chars();
        let action = chars.next().unwrap();
        let number = chars
            .into_iter()
            .collect::<String>()
            .parse::<isize>()
            .unwrap();

        match (action, number) {
            // Turn actions
            ('L', 90) => Action::Turn(Turn::Left),
            ('L', 180) => Action::Turn(Turn::Around),
            ('L', 270) => Action::Turn(Turn::Right),
            ('R', 90) => Action::Turn(Turn::Right),
            ('R', 180) => Action::Turn(Turn::Around),
            ('R', 270) => Action::Turn(Turn::Left),

            // Move actions
            ('N', number) => Action::Move(Direction::North, number),
            ('S', number) => Action::Move(Direction::South, number),
            ('E', number) => Action::Move(Direction::East, number),
            ('W', number) => Action::Move(Direction::West, number),
            ('F', number) => Action::Forward(number),

            _ => panic!("Unknown action '{action}' '{number}'"),
        }
    })
}

pub fn part_one(input: &str) -> Option<isize> {
    let (_, ship) = parse_input(input).fold(
        (Direction::East, Position::new(0, 0)),
        |(direction, ship), action| match action {
            Action::Turn(turn) => (direction.turn(turn), ship),
            Action::Move(move_direction, amount) => {
                (direction, ship.transition(move_direction, amount))
            }
            Action::Forward(amount) => (direction, ship.transition(direction, amount)),
        },
    );

    Some(ship.x.abs() + ship.y.abs())
}

pub fn part_two(input: &str) -> Option<isize> {
    let (ship, _) = parse_input(input).fold(
        (Position::new(0, 0), RelativePosition::new(10, 1)),
        |(ship, waypoint), action| match action {
            Action::Turn(turn) => (ship, waypoint.turn(turn)),
            Action::Move(direction, amount) => (ship, waypoint.transition(direction, amount)),
            Action::Forward(amount) => (ship.move_towards(waypoint, amount), waypoint),
        },
    );

    Some(ship.x.abs() + ship.y.abs())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(25));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(286));
    }
}
