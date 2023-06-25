use bitvector::*;

#[derive(Debug)]
struct Ferry {
    seats: BitVector,
    occupied_seats: BitVector,
    width: usize,
    height: usize,
    size: usize,
}

impl Ferry {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<bool>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        'L' => true,
                        '.' => false,
                        '#' => panic!("Did not expected an occupied seat in the input"),
                        _ => panic!(""),
                    })
                    .collect()
            })
            .collect();

        let width = grid[0].len();
        let height = grid.len();
        let size = width * height;
        let mut seats = BitVector::new(size);

        for (y, row) in grid.iter().enumerate() {
            for (x, is_seat) in row.iter().enumerate() {
                if !is_seat {
                    continue;
                }

                seats.insert(x + y * width);
            }
        }

        Self {
            seats,
            occupied_seats: BitVector::new(size),
            height,
            width,
            size,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn simulate(&mut self, tolerance: usize, adjacent_seat_masks: &[(usize, BitVector)]) {
        let mut is_modified = true;

        while is_modified {
            let mut updated_occupied_seats = self.occupied_seats.clone();

            for (seat, mask) in adjacent_seat_masks {
                let number_of_occupied_adjacent_seats =
                    mask.intersection(&self.occupied_seats).len();
                let current_seat_is_occupied = self.occupied_seats.contains(*seat);

                match (current_seat_is_occupied, number_of_occupied_adjacent_seats) {
                    // If a seat is empty and there are no occupied seats adjacent to it, the seat becomes occupied.
                    (false, 0) => assert!(updated_occupied_seats.insert(*seat)),
                    (false, _) => continue,
                    // If a seat is occupied and X or more seats adjacent to it are also occupied, the seat becomes empty.
                    (true, amount) if amount >= tolerance => {
                        assert!(updated_occupied_seats.remove(*seat))
                    }
                    (true, _) => continue,
                };
            }

            is_modified = updated_occupied_seats != self.occupied_seats;
            self.occupied_seats = updated_occupied_seats;
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut ferry = Ferry::new(input);

    let adjacent_seat_masks: Vec<(usize, BitVector)> = (0..ferry.height)
        .flat_map(move |y| (0..ferry.width).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            let index = ferry.index(x as usize, y as usize);

            if !ferry.seats.contains(index) {
                return None;
            }

            let x = x as isize;
            let y = y as isize;

            let adjacent_seats = [
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x - 1, y - 1),
                (x, y - 1),
                (x, y + 1),
            ];

            let mut mask = BitVector::new(ferry.size);

            for (x, y) in adjacent_seats {
                if x < 0 || x >= ferry.width as isize {
                    continue;
                }

                if y < 0 || y >= ferry.height as isize {
                    continue;
                }

                let index = ferry.index(x as usize, y as usize);
                if !ferry.seats.contains(index) {
                    continue;
                }

                assert!(mask.insert(index));
            }

            Some((ferry.index(x as usize, y as usize), mask))
        })
        .collect();

    ferry.simulate(4, &adjacent_seat_masks);

    Some(ferry.occupied_seats.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut ferry = Ferry::new(input);

    let adjacent_seat_masks: Vec<(usize, BitVector)> = (0..ferry.height)
        .flat_map(move |y| (0..ferry.width).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            let index = ferry.index(x as usize, y as usize);

            if !ferry.seats.contains(index) {
                return None;
            }

            let adjacent_seats_directions = [
                (1, 0),
                (1, 1),
                (1, -1),
                (-1, 0),
                (-1, 1),
                (-1, -1),
                (0, -1),
                (0, 1),
            ];

            let mut mask = BitVector::new(ferry.size);

            for (dx, dy) in adjacent_seats_directions {
                let mut x = x as isize;
                let mut y = y as isize;

                let mut found_seat = false;
                let mut is_in_feasible_range = true;

                while !found_seat && is_in_feasible_range {
                    x += dx;
                    y += dy;

                    if x < 0 || x >= ferry.width as isize {
                        is_in_feasible_range = false;
                        break;
                    }

                    if y < 0 || y >= ferry.height as isize {
                        is_in_feasible_range = false;
                        break;
                    }

                    found_seat = ferry.seats.contains(ferry.index(x as usize, y as usize))
                }

                if !is_in_feasible_range {
                    continue;
                }

                let index = ferry.index(x as usize, y as usize);
                if !ferry.seats.contains(index) {
                    continue;
                }

                assert!(mask.insert(index));
            }

            Some((ferry.index(x as usize, y as usize), mask))
        })
        .collect();

    ferry.simulate(5, &adjacent_seat_masks);

    Some(ferry.occupied_seats.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(37));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(26));
    }
}
