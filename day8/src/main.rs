#[derive(Debug)]
enum Error {
    ParseError(String),
    ConditionalParseError(String),
    OperationParseError(String),
}

struct Computer {
    registers: std::collections::HashMap<String, i32>,
    instructions: Vec<Instruction>,
    pointer: usize,
    max_register: i32,
}

impl TryFrom<&str> for Computer {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let instructions = value
            .lines()
            .map(|line| line.try_into())
            .collect::<Result<Vec<Instruction>, Error>>()?;

        Ok(Self {
            instructions,
            registers: std::collections::HashMap::new(),
            pointer: 0,
            max_register: 0,
        })
    }
}

impl Computer {
    fn step(&mut self) {
        let val = self.instructions[self.pointer].call(&mut self.registers);
        if &self.max_register < val {
            self.max_register = *val;
        }
    }

    fn run(&mut self) {
        for _ in 0..self.instructions.len() {
            self.step();
            self.pointer += 1;
        }
    }

    fn scan(&self) -> i32 {
        *self.registers.iter().map(|(k, v)| v).max().expect("no max")
    }
}

enum Operator {
    Dec,
    Inc,
}

impl TryFrom<&str> for Operator {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "dec" => Operator::Dec,
            "inc" => Operator::Inc,
            _ => return Err(Error::OperationParseError("unkown operator".into())),
        })
    }
}

enum Condition {
    LessThan,
    LessThanEqual,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
}

impl TryFrom<&str> for Condition {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Condition::*;

        Ok(match value {
            "<" => LessThan,
            "<=" => LessThanEqual,
            "==" => Equal,
            "!=" => NotEqual,
            ">" => GreaterThan,
            ">=" => GreaterThanEqual,
            _ => return Err(Error::ConditionalParseError("unknown condition".into())),
        })
    }
}

struct Conditional {
    lh: String,
    rh: i32,
    condition: Condition,
}

impl TryFrom<&str> for Conditional {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split(" ");
        let lh = iter
            .next()
            .ok_or_else(|| Error::ConditionalParseError("no lh".into()))?
            .to_string();
        let condition: Condition = iter
            .next()
            .ok_or_else(|| Error::ConditionalParseError("no condition".into()))?
            .try_into()?;
        let rh = iter
            .next()
            .ok_or_else(|| Error::ConditionalParseError("no rh".into()))?
            .parse()
            .map_err(|_| Error::ConditionalParseError("no rh".into()))?;

        Ok(Self { lh, condition, rh })
    }
}

struct Operation {
    register: String,
    operator: Operator,
    value: i32,
}

impl Operation {
    fn call<'reg>(&self, registers: &'reg mut std::collections::HashMap<String, i32>) -> &'reg i32 {
        self.operator
            .call(registers.entry(self.register.clone()), self.value);

        registers.get(&self.register).unwrap()
    }
}

impl TryFrom<&str> for Operation {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split(" ");
        let register = iter
            .next()
            .ok_or_else(|| Error::OperationParseError("no register".into()))?
            .to_string();
        let operator: Operator = iter
            .next()
            .ok_or_else(|| Error::OperationParseError("no register".into()))?
            .try_into()?;
        let value = iter
            .next()
            .ok_or_else(|| Error::OperationParseError("no value".into()))?
            .parse()
            .map_err(|_| Error::OperationParseError("no value".into()))?;

        Ok(Self {
            register,
            operator,
            value,
        })
    }
}

struct Instruction {
    operation: Operation,
    conditional: Conditional,
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (operation, conditional) = value
            .split_once(" if ")
            .ok_or_else(|| Error::ParseError("no if statement".into()))?;
        Ok(Self {
            operation: operation.try_into()?,
            conditional: conditional.try_into()?,
        })
    }
}

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");
    let mut computer: Computer = input.try_into()?;
    computer.run();
    let part1 = computer.scan();
    println!("part1: {}", part1);
    println!("part2: {}", computer.max_register);

    Ok(())
}

impl Instruction {
    fn call<'reg>(&self, registers: &'reg mut std::collections::HashMap<String, i32>) -> &'reg i32 {
        if self.conditional.call(registers) {
            self.operation.call(registers)
        } else {
            &0
        }
    }
}
impl Conditional {
    fn call(&self, registers: &std::collections::HashMap<String, i32>) -> bool {
        let lh = registers.get(&self.lh).unwrap_or(&0);
        self.condition.call(lh, &self.rh)
    }
}
impl Condition {
    fn call(&self, lh: &i32, rh: &i32) -> bool {
        match self {
            Condition::LessThan => lh < rh,
            Condition::LessThanEqual => lh <= rh,
            Condition::Equal => lh == rh,
            Condition::NotEqual => lh != rh,
            Condition::GreaterThan => lh > rh,
            Condition::GreaterThanEqual => lh >= rh,
        }
    }
}
impl Operator {
    fn call(&self, register: std::collections::hash_map::Entry<String, i32>, value: i32) {
        match self {
            Operator::Dec => *register.or_insert(0) -= value,
            Operator::Inc => *register.or_insert(0) += value,
        }
    }
}
