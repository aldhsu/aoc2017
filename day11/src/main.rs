enum Direction {
    South,
    SouthWest,
    SouthEast,
    North,
    NorthWest,
    NorthEast,
}

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        use Direction::*;
        match input {
            "s" => South,
            "sw" => SouthWest,
            "se" => SouthEast,
            "n" => North,
            "nw" => NorthWest,
            "ne" => NorthEast,
            _ => unreachable!()
        }
    }
}

type Coord = (i32, i32);


impl Direction {
    fn apply(&self, coord: Coord) -> Coord {
        match self {
            Direction::South => (coord.0, coord.1 + 2),
            Direction::SouthWest => (coord.0 - 1, coord.1 + 1),
            Direction::SouthEast => (coord.0 + 1, coord.1 + 1),
            Direction::North => (coord.0, coord.1 - 2),
            Direction::NorthWest => (coord.0 - 1, coord.1 - 1),
            Direction::NorthEast => (coord.0 + 1, coord.1 - 1),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn direction_apply_works() {
        assert_eq!(part1("ne,ne,sw,sw"), (0, 0));
        assert_eq!(part1("se,sw,se,sw,sw"), (0, 0));
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.trim().split(',').map(|d| d.into()).collect()
}

fn part1(input: &str) -> Coord {
    let directions = parse_input(input);
    directions.iter().fold((0,0), |memo, direction| direction.apply(memo))
}

fn part2(input: &str) -> i32 {
    let directions = parse_input(input);
    let (_, max) = directions.iter().fold(((0,0), 0), |(coord, max), direction| {
        let new_coord = direction.apply(coord);
        let distance = new_coord.0.abs() + (new_coord.1.abs() - new_coord.0.abs()).max(0) / 2;

        (new_coord, max.max(distance))
    });
    max
}

fn main() {
    let input = include_str!("../input.txt");

    println!("part1: {:?}", part1(input));
    println!("part1: {:?}", part2(input));
}
