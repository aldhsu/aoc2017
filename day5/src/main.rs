use std::marker::PhantomData;
struct Stepper1;
struct Stepper3;

struct Computer<T> {
    registers: Vec<i32>,
    current_position: i32,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
enum Error {
    ParseError,
}

impl<T> TryFrom<&str> for Computer<T> {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let registers = input
            .lines()
            .map(|line| {
                line.parse::<i32>().map_err(|e| {
                    dbg!(e);
                    Error::ParseError
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Computer {
            registers,
            current_position: 0,
            phantom: PhantomData,
        })
    }
}

trait Stepper {
    fn step(&mut self) -> Option<usize>;
}

impl Stepper for Computer<Stepper1> {
    fn step(&mut self) -> Option<usize> {
        let new_position = self.current_position + self.registers[self.current_position as usize];
        self.registers[self.current_position as usize] += 1;

        if new_position <= self.registers.len() as i32 {
            self.current_position = new_position;
            Some(new_position as usize)
        } else {
            None
        }
    }
}

impl<T> Computer<T>
where
    Computer<T>: Stepper,
{
    fn run(&mut self) -> usize {
        let mut steps: usize = 1;
        while self.step().is_some() {
            steps += 1;
        }
        steps
    }
}

impl Stepper for Computer<Stepper3> {
    fn step(&mut self) -> Option<usize> {
        let current_value = self.registers[self.current_position as usize];
        let new_position = self.current_position + current_value;

        let register_change = if current_value >= 3 { -1 } else { 1 };
        self.registers[self.current_position as usize] += register_change;

        if new_position < self.registers.len() as i32 {
            self.current_position = new_position;
            Some(new_position as usize)
        } else {
            None
        }
    }
}

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");
    let mut computer: Computer<Stepper1> = Computer::try_from(input)?;
    let part1 = computer.run();
    println!("part 1: {}", part1);

    let mut computer: Computer<Stepper3> = Computer::try_from(input)?;
    let part2 = computer.run();
    println!("part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"0
3
0
1
-3"#;

        let mut computer: Computer<Stepper1> = Computer::try_from(input).unwrap();
        assert_eq!(computer.run(), 5);
    }
}
