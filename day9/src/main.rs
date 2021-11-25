extern crate nom;
use std::error::Error;

use nom::branch::alt;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded};
use nom::{
    bytes::complete::{is_not, tag, take},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Part {
    Group(Option<Vec<Part>>),
    Garbage(usize),
}

impl Part {
    pub fn score(&self) -> usize {
        self._score(1)
    }

    fn _score(&self, count: usize) -> usize {
        match self {
            Part::Group(Some(children)) => {
                children
                    .iter()
                    .map(|child| child._score(count + 1))
                    .sum::<usize>()
                    + count
            }
            Part::Group(None) => count,
            _ => 0,
        }
    }

    pub fn collect_garbage(&self) -> usize {
        match self {
            Part::Group(Some(children)) => children.iter().map(Self::collect_garbage).sum(),
            Part::Garbage(size) => *size,
            _ => 0,
        }
    }
}

fn parse_stream(input: &str) -> IResult<&str, Part> {
    group(input)
}

fn bang(input: &str) -> IResult<&str, usize> {
    // in garbage ! ignores next
    let (input, _) = preceded(nom::character::complete::char('!'), take(1usize))(input)?;
    Ok((input, 0))
}

fn not_bang_or_close(input: &str) -> IResult<&str, usize> {
    let (input, item) = is_not(">!")(input)?;
    Ok((input, item.len()))
}

fn garbage(input: &str) -> IResult<&str, Part> {
    //garbage begins with <
    //garbage ends with >
    let (input, parts) = delimited(tag("<"), many0(alt((bang, not_bang_or_close))), tag(">"))(input)?;
    Ok((input, Part::Garbage(parts.iter().sum())))
}

fn inner_item(input: &str) -> IResult<&str, Part> {
    alt((garbage, group))(input)
}

fn inner_group(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list0(tag(","), inner_item)(input)
}

fn group(input: &str) -> IResult<&str, Part> {
    let (input, inner) = delimited(tag("{"), inner_group, tag("}"))(input)?;
    if !inner.is_empty() {
        Ok((input, Part::Group(Some(inner))))
    } else {
        Ok((input, Part::Group(None)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_with_bang() {
        let input = "!>";
        assert_eq!(bang(input), Ok(("", 0)));
    }

    #[test]
    fn it_works_with_garbage() {
        let input = "<>";
        assert_eq!(garbage(input), Ok(("", Part::Garbage(0))));

        let input = "<random characters>";
        assert_eq!(garbage(input), Ok(("", Part::Garbage(17))));

        let input = "<<<<>";
        assert_eq!(garbage(input), Ok(("", Part::Garbage(3))));

        let input = "<{!>>";
        assert_eq!(garbage(input), Ok(("", Part::Garbage(1))));

        let input = "<!!>";
        assert_eq!(garbage(input), Ok(("", Part::Garbage(0))));

        let input = "<!!!>>";
        assert_eq!(garbage(input), Ok(("", Part::Garbage(0))));

        let input = r#"<{o"i!a,<{i<a>"#;
        assert_eq!(garbage(input), Ok(("", Part::Garbage(10))));

        let input = "<a!>>";
        assert_eq!(garbage(input), Ok(("", Part::Garbage(1))));
    }

    #[test]
    fn it_works_splitting_garbage() {
        let input = "<>,<>";
        assert_eq!(
            inner_group(input),
            Ok(("", vec![Part::Garbage(0), Part::Garbage(0)]))
        );
    }

    #[test]
    fn it_can_parse_a_group() {
        let input = "{}";
        assert_eq!(group(input), Ok(("", Part::Group(None))));
    }

    #[test]
    fn it_can_get_an_inner_item() {
        let input = "{}";
        assert_eq!(inner_item(input), Ok(("", Part::Group(None))));

        let input = "{{}},{}";
        assert_eq!(
            inner_group(input),
            Ok((
                "",
                vec![
                    Part::Group(Some(vec![Part::Group(None)])),
                    Part::Group(None)
                ]
            ))
        );
    }

    #[test]
    fn it_works_splitting_groups() {
        let input = "{},{}";
        assert_eq!(
            inner_group(input),
            Ok(("", vec![Part::Group(None), Part::Group(None)]))
        );

        let input = "{{}},{}";
        assert_eq!(
            inner_group(input),
            Ok((
                "",
                vec![
                    Part::Group(Some(vec![Part::Group(None)])),
                    Part::Group(None)
                ]
            ))
        );
    }

    #[test]
    fn it_can_find_score() {
        let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
        let (_, root) = parse_stream(input).unwrap();
        assert_eq!(root.score(), 9);

        let input = "{{{},{},{{}}}}";
        let (_, root) = parse_stream(input).unwrap();
        assert_eq!(root.score(), 16);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt").trim();
    let (_, root) = parse_stream(input)?;

    let part1 = root.score();
    println!("part1: {}", part1);
    let part2 = root.collect_garbage();
    println!("part2: {}", part2);
    Ok(())
}
