#[derive(Debug)]
pub enum Error {
    ParseError,
}

fn parse_banks(input: &str) -> Result<Vec<u8>, Error> {
    input
        .split("\t")
        .map(|num| num.parse().map_err(|_| Error::ParseError))
        .collect()
}

mod Solver {
    pub use crate::Error;

    pub fn find_loop(mut banks: Vec<u8>) -> Result<(Vec<u8>, usize), Error> {
        let mut set: std::collections::HashSet<Vec<u8>> = std::collections::HashSet::new();
        set.insert(banks.clone());

        let mut loop_count = 1;

        loop {
            let max = *banks.iter().max().expect("no max");
            let index = banks.iter().position(|&a| a == max).unwrap();
            banks[index] = 0;

            // increase all banks by divisible amount
            let banks_increase = max as usize / banks.len();
            banks.iter_mut().for_each(|b| *b += banks_increase as u8);

            // increase all banks after max by 1 for remainder
            let remainder = max as usize % banks.len();
            for n in 0..remainder {
                let pos = (index + n as usize + 1) % banks.len();
                banks[pos] += 1;
            }

            if !set.insert(banks.clone()) {
                break;
            } else {
                loop_count += 1;
            }
        }

        Ok((banks, loop_count))
    }

    pub fn find_repeat(mut banks: Vec<u8>, sig: Vec<u8>) -> Result<(Vec<u8>, usize), Error> {
        let mut loop_count = 0;

        loop {
            let max = *banks.iter().max().expect("no max");
            let index = banks.iter().position(|&a| a == max).unwrap();
            banks[index] = 0;

            // increase all banks by divisible amount
            let banks_increase = max as usize / banks.len();
            banks.iter_mut().for_each(|b| *b += banks_increase as u8);

            // increase all banks after max by 1 for remainder
            let remainder = max as usize % banks.len();
            for n in 0..remainder {
                let pos = (index + n as usize + 1) % banks.len();
                banks[pos] += 1;
            }

            loop_count += 1;
            if banks == sig {
                break;
            }
        }

        Ok((banks, loop_count))
    }

    pub fn part1(banks: Vec<u8>) -> Result<usize, Error> {
        let (_, count) = find_loop(banks)?;
        Ok(count)
    }

    pub fn part2(banks: Vec<u8>) -> Result<usize, Error> {
        let (banks, _) = find_loop(banks)?;
        let (_, count) = find_repeat(banks.clone(), banks)?;
        Ok(count)
    }

    #[cfg(test)]
    mod test {
        use crate::{parse_banks, Solver};

        #[test]
        fn part1() {
            let input = "0	2	7	0";
            let banks = parse_banks(input).unwrap();
            let part1 = Solver::part1(banks);
            assert_eq!(part1.unwrap(), 5);
        }

        #[test]
        fn part2() {
            let input = "0	2	7	0";
            let banks = parse_banks(input).unwrap();
            let part1 = Solver::part2(banks);
            assert_eq!(part1.unwrap(), 4);
        }
    }
}

fn main() -> Result<(), Error> {
    let input = "4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3";
    let banks = parse_banks(input)?;
    let part1 = Solver::part1(banks.clone());
    let part2 = Solver::part2(banks);

    println!("part1: {}", part1?);
    println!("part1: {}", part2?);

    Ok(())
}
