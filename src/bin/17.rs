use rustc_hash::FxHashSet;

type Scalar = i8;
type Coordinate = (Scalar, Scalar, Scalar);
type HyperCoordinate = (Scalar, Scalar, Scalar, Scalar);

fn neighbours(x: Scalar, y: Scalar, z: Scalar) -> [Coordinate; 27] {
    [
        (x, y, z),
        (x, y, z - 1),
        (x, y, z + 1),
        (x, y - 1, z),
        (x, y - 1, z - 1),
        (x, y - 1, z + 1),
        (x, y + 1, z),
        (x, y + 1, z - 1),
        (x, y + 1, z + 1),
        (x - 1, y, z),
        (x - 1, y, z - 1),
        (x - 1, y, z + 1),
        (x - 1, y - 1, z),
        (x - 1, y - 1, z - 1),
        (x - 1, y - 1, z + 1),
        (x - 1, y + 1, z),
        (x - 1, y + 1, z - 1),
        (x - 1, y + 1, z + 1),
        (x + 1, y, z),
        (x + 1, y, z - 1),
        (x + 1, y, z + 1),
        (x + 1, y - 1, z),
        (x + 1, y - 1, z - 1),
        (x + 1, y - 1, z + 1),
        (x + 1, y + 1, z),
        (x + 1, y + 1, z - 1),
        (x + 1, y + 1, z + 1),
    ]
}

fn hyper_neighbours(x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> [HyperCoordinate; 81] {
    [
        (x, y, z, w),
        (x, y, z, w - 1),
        (x, y, z, w + 1),
        (x, y, z - 1, w),
        (x, y, z - 1, w - 1),
        (x, y, z - 1, w + 1),
        (x, y, z + 1, w),
        (x, y, z + 1, w - 1),
        (x, y, z + 1, w + 1),
        (x, y - 1, z, w),
        (x, y - 1, z, w - 1),
        (x, y - 1, z, w + 1),
        (x, y - 1, z - 1, w),
        (x, y - 1, z - 1, w - 1),
        (x, y - 1, z - 1, w + 1),
        (x, y - 1, z + 1, w),
        (x, y - 1, z + 1, w - 1),
        (x, y - 1, z + 1, w + 1),
        (x, y + 1, z, w),
        (x, y + 1, z, w - 1),
        (x, y + 1, z, w + 1),
        (x, y + 1, z - 1, w),
        (x, y + 1, z - 1, w - 1),
        (x, y + 1, z - 1, w + 1),
        (x, y + 1, z + 1, w),
        (x, y + 1, z + 1, w - 1),
        (x, y + 1, z + 1, w + 1),
        (x - 1, y, z, w),
        (x - 1, y, z, w - 1),
        (x - 1, y, z, w + 1),
        (x - 1, y, z - 1, w),
        (x - 1, y, z - 1, w - 1),
        (x - 1, y, z - 1, w + 1),
        (x - 1, y, z + 1, w),
        (x - 1, y, z + 1, w - 1),
        (x - 1, y, z + 1, w + 1),
        (x - 1, y - 1, z, w),
        (x - 1, y - 1, z, w - 1),
        (x - 1, y - 1, z, w + 1),
        (x - 1, y - 1, z - 1, w),
        (x - 1, y - 1, z - 1, w - 1),
        (x - 1, y - 1, z - 1, w + 1),
        (x - 1, y - 1, z + 1, w),
        (x - 1, y - 1, z + 1, w - 1),
        (x - 1, y - 1, z + 1, w + 1),
        (x - 1, y + 1, z, w),
        (x - 1, y + 1, z, w - 1),
        (x - 1, y + 1, z, w + 1),
        (x - 1, y + 1, z - 1, w),
        (x - 1, y + 1, z - 1, w - 1),
        (x - 1, y + 1, z - 1, w + 1),
        (x - 1, y + 1, z + 1, w),
        (x - 1, y + 1, z + 1, w - 1),
        (x - 1, y + 1, z + 1, w + 1),
        (x + 1, y, z, w),
        (x + 1, y, z, w - 1),
        (x + 1, y, z, w + 1),
        (x + 1, y, z - 1, w),
        (x + 1, y, z - 1, w - 1),
        (x + 1, y, z - 1, w + 1),
        (x + 1, y, z + 1, w),
        (x + 1, y, z + 1, w - 1),
        (x + 1, y, z + 1, w + 1),
        (x + 1, y - 1, z, w),
        (x + 1, y - 1, z, w - 1),
        (x + 1, y - 1, z, w + 1),
        (x + 1, y - 1, z - 1, w),
        (x + 1, y - 1, z - 1, w - 1),
        (x + 1, y - 1, z - 1, w + 1),
        (x + 1, y - 1, z + 1, w),
        (x + 1, y - 1, z + 1, w - 1),
        (x + 1, y - 1, z + 1, w + 1),
        (x + 1, y + 1, z, w),
        (x + 1, y + 1, z, w - 1),
        (x + 1, y + 1, z, w + 1),
        (x + 1, y + 1, z - 1, w),
        (x + 1, y + 1, z - 1, w - 1),
        (x + 1, y + 1, z - 1, w + 1),
        (x + 1, y + 1, z + 1, w),
        (x + 1, y + 1, z + 1, w - 1),
        (x + 1, y + 1, z + 1, w + 1),
    ]
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut active_cubes: FxHashSet<Coordinate> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, value)| *value == '#')
                .map(move |(x, _)| (x as Scalar, y as Scalar, 0))
        })
        .collect();

    for _ in 0..6 {
        active_cubes = active_cubes
            .iter()
            .copied()
            .flat_map(|(x, y, z)| neighbours(x, y, z))
            .filter(|(x, y, z)| {
                // Note: the cube itself is included in the neighbour list.
                let active_neighbours = neighbours(*x, *y, *z)
                    .into_iter()
                    .filter(|cube| active_cubes.contains(cube))
                    .count();

                if active_cubes.contains(&(*x, *y, *z)) {
                    // If a cube is active and exactly 2 or 3 of its neighbors are also active,
                    // the cube remains active. Otherwise, the cube becomes inactive.
                    active_neighbours == 3 || active_neighbours == 4
                } else {
                    // If a cube is inactive but exactly 3 of its neighbors are active,
                    // the cube becomes active. Otherwise, the cube remains inactive.
                    active_neighbours == 3
                }
            })
            .collect();
    }

    Some(active_cubes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut active_cubes: FxHashSet<HyperCoordinate> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, value)| *value == '#')
                .map(move |(x, _)| (x as Scalar, y as Scalar, 0, 0))
        })
        .collect();

    for _ in 0..6 {
        active_cubes = active_cubes
            .iter()
            .copied()
            .flat_map(|(x, y, z, w)| hyper_neighbours(x, y, z, w))
            .filter(|(x, y, z, w)| {
                // Note: the cube itself is included in the neighbour list.
                let active_neighbours = hyper_neighbours(*x, *y, *z, *w)
                    .into_iter()
                    .filter(|cube| active_cubes.contains(cube))
                    .count();

                if active_cubes.contains(&(*x, *y, *z, *w)) {
                    // If a cube is active and exactly 2 or 3 of its neighbors are also active,
                    // the cube remains active. Otherwise, the cube becomes inactive.
                    active_neighbours == 3 || active_neighbours == 4
                } else {
                    // If a cube is inactive but exactly 3 of its neighbors are active,
                    // the cube becomes active. Otherwise, the cube remains inactive.
                    active_neighbours == 3
                }
            })
            .collect();
    }

    Some(active_cubes.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(112));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(848));
    }
}
