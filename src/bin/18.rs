use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Clone)]
enum Expression {
    Number(usize),
    Operation(Operation),
    Group(Vec<Expression>),
}

fn parse_expression(input: &str) -> Expression {
    let mut characters = input.chars().filter(|char| char != &' ').peekable();
    let mut stack: VecDeque<Vec<Expression>> = VecDeque::new();
    let mut expressions: Vec<Expression> = Vec::new();

    while let Some(char) = characters.next() {
        match char {
            '(' => {
                stack.push_back(expressions);
                expressions = Vec::new();
            }
            ')' => {
                let group = Expression::Group(expressions);
                expressions = stack.pop_back().unwrap();
                expressions.push(group);
            }
            '+' => expressions.push(Expression::Operation(Operation::Add)),
            '*' => expressions.push(Expression::Operation(Operation::Multiply)),
            _ => {
                let number = char.to_string().parse::<usize>().unwrap();
                expressions.push(Expression::Number(number));
            }
        }
    }

    Expression::Group(expressions)
}

fn solve_expression(expression: Expression) -> usize {
    match expression {
        Expression::Number(value) => value,
        Expression::Operation(_) => panic!(),
        Expression::Group(expressions) => {
            let mut expressions = expressions.into_iter();
            let mut accumulator = match expressions.next().unwrap() {
                Expression::Operation(_) => panic!(),
                Expression::Number(number) => number,
                Expression::Group(expressions) => solve_expression(Expression::Group(expressions)),
            };

            while let Some(expression) = expressions.next() {
                let Expression::Operation(operation) = expression else {
                    panic!("Expected an operation after a group or number");
                };

                let other = match expressions.next().unwrap() {
                    Expression::Operation(_) => panic!("After an operation, there must be a group or a number. Not another operation."),
                    Expression::Number(number) => number,
                    Expression::Group(expressions) => solve_expression(Expression::Group(expressions)),
                };

                accumulator = match operation {
                    Operation::Multiply => accumulator * other,
                    Operation::Add => accumulator + other,
                };
            }

            accumulator
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| parse_expression(line))
            .map(|expression| solve_expression(expression))
            .sum(),
    )
}

fn process_expression(expression: Expression) -> Expression {
    match expression {
        Expression::Operation(_) => panic!(),
        Expression::Number(_) => expression,
        Expression::Group(expressions) => {
            let mut expressions = expressions.into_iter();
            let mut processed_expressions: Vec<Expression> = Vec::new();
            let mut previous_expression = None;

            while let Some(expression) = expressions.next() {
                match expression {
                    Expression::Number(_) => previous_expression = Some(expression),
                    Expression::Group(_) => {
                        previous_expression = Some(process_expression(expression))
                    }
                    Expression::Operation(Operation::Multiply) => {
                        processed_expressions.push(previous_expression.clone().unwrap());
                        processed_expressions.push(expression);
                    }
                    Expression::Operation(Operation::Add) => {
                        previous_expression = Some(Expression::Group(vec![
                            previous_expression.clone().unwrap(),
                            expression,
                            process_expression(expressions.next().unwrap()),
                        ]));
                    }
                }
            }

            processed_expressions.push(previous_expression.unwrap());

            Expression::Group(processed_expressions)
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| parse_expression(line))
            .map(|expression| process_expression(expression))
            .map(|expression| solve_expression(expression))
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_part_one() {
        assert_eq!(part_one("1 + 2 * 3 + 4 * 5 + 6"), Some(71));
        assert_eq!(part_one("1 + (2 * 3) + (4 * (5 + 6))"), Some(51));
        assert_eq!(part_one("2 * 3 + (4 * 5)"), Some(26));
        assert_eq!(part_one("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Some(437));
        assert_eq!(part_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), Some(12240));
        assert_eq!(part_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), Some(13632));
    }

    #[test]
    #[rustfmt::skip]
    fn test_part_two() {
        assert_eq!(part_two("2 * 3 + 4 * 5"), Some(70));
        assert_eq!(part_two("1 + 2 * 3 + 4 * 5 + 6"), Some(231));
        assert_eq!(part_two("1 + (2 * 3) + (4 * (5 + 6))"), Some(51));
        assert_eq!(part_two("2 * 3 + (4 * 5)"), Some(46));
        assert_eq!(part_two("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Some(1445));
        assert_eq!(part_two("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), Some(669060));
        assert_eq!(part_two("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), Some(23340));
    }
}
