mod solver {

    #[derive(Debug)]
    pub enum Error {
        Part1Error,
    }

    pub fn part1(input: f64) -> Result<u64, Error> {
        let sqrt = input.sqrt();
        let layer = {
            let mut side = sqrt.ceil() as u64;
            if side % 2 == 0 {
                side += 1;
            }
            (side - 1) / 2
        };

        fn layer_fast_travel_point(layer: i64, n: i64) -> i64 {
            let offset = (2 * layer) * n;
            let previous_count = ((layer - 1) * 2 + 1).pow(2) - (layer);

            offset + previous_count
        }

        let layer_delta = (1..5_i64)
            .map(|i| layer_fast_travel_point(layer as i64, i))
            .map(|point| (input as i64 - point as i64).abs())
            .min()
            .unwrap();

        let layer = layer as i64;

        Ok(layer as u64 + layer_delta as u64)
    }

    #[cfg(test)]
    mod test {
        use super::part1;

        #[test]
        fn it_works() {
            assert_eq!(2, part1(23.0).unwrap());
            assert_eq!(6, part1(31.0).unwrap());
            assert_eq!(31, part1(1024.0).unwrap());
        }
    }

    #[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
    struct Coord(i32, i32);
    impl std::ops::Add for Coord {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl std::ops::Add for &Coord {
        type Output = Coord;

        fn add(self, rhs: Self) -> Self::Output {
            Coord(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl Coord {
        const SURROUNDING: [Coord; 8] = [
            Coord(-1, 1),
            Coord(0, 1),
            Coord(1, 1),
            Coord(-1, 0),
            Coord(1, 0),
            Coord(-1, -1),
            Coord(0, -1),
            Coord(1, -1),
        ];

        fn surrounding(&self, hash: &std::collections::HashMap<Coord, u64>) -> u64 {
            Self::SURROUNDING
                .iter()
                .filter_map(|offset| hash.get(&(offset + self)))
                .sum::<u64>()
        }
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Cell(Coord, u32);

    impl Cell {
        fn next(&self) -> Self {
            Cell(self.next_coord(), self.1 + 1)
        }

        fn next_coord(&self) -> Coord {
            let count = self.1;
            let sqrt = (count as f64).sqrt();
            let layer = {
                let mut side = sqrt.ceil() as u64;
                if side % 2 == 0 {
                    side += 1;
                }
                (side - 1) / 2
            };

            let side = layer * 2;
            let last_total = ((layer - 1) * 2 + 1).pow(2);

            let delta = match (count - last_total as u32) / side as u32 {
                0 => (1, 0),
                1 => (0, -1_i32),
                2 => (-1, 0),
                3 | 4 => (0, 1),
                _ => unreachable!(),
            };

            Coord(self.0 .0 + delta.0, self.0 .1 + delta.1)
        }

        fn coord(&self) -> Coord {
            self.0
        }
    }

    #[cfg(test)]
    mod cell_test {
        use super::Cell;
        use crate::solver::Coord;

        #[test]
        fn next_works() {
            assert_eq!(Cell(Coord(0, 0), 1).next(), Cell(Coord(1, 0), 2));
            assert_eq!(Cell(Coord(0, 0), 40).next(), Cell(Coord(0, 1), 41));
        }
    }

    pub fn part2(input: u64) -> Result<u64, Error> {
        let mut hash: std::collections::HashMap<Coord, u64> = std::collections::HashMap::new();
        hash.insert(Coord(0, 0), 1);
        let mut value = 1_u64;
        let mut current_cell = Cell(Coord(0, 1), 2);

        while value < input {
            let coord = current_cell.coord();
            value = coord.surrounding(&hash);
            hash.insert(coord, value);
            current_cell = current_cell.next();
        }

        Ok(value)
    }
}

fn main() {
    let input = 265149;
    println!("part1: {}", solver::part1(input as f64).unwrap());
    println!("part2: {}", solver::part2(input).unwrap());
}
