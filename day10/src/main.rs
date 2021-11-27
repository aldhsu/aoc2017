use core::fmt;

#[derive(Debug)]
enum Error {
    ParseError,
}

fn part1(input: &str, buffer_length: i32) -> Result<i32, Error> {
    let lengths = input
        .split(',')
        .map(|num| num.parse().map_err(|_| Error::ParseError))
        .collect::<Result<Vec<i32>, Error>>()?;
    let mut current_position = 0;
    let mut buffer = (0..buffer_length).collect::<Vec<_>>();

    for (skip_size, length) in lengths.into_iter().enumerate() {
        let mut temp_buffer = buffer.iter().cycle();

        // replace with advance_by
        for _ in 0..current_position {
            temp_buffer.next();
        }

        let buffer_copy = temp_buffer
            .take(length as usize)
            .cloned()
            .collect::<Vec<i32>>();

        for num in buffer_copy.iter().rev() {
            buffer[current_position as usize % buffer_length as usize] = *num;
            current_position += 1;
        }
        current_position += skip_size;
    }

    Ok(buffer[0] * buffer[1])
}
fn part2(input: &str, buffer_length: i32) -> Result<String, Error> {
    let lengths = input
        .chars()
        .map(|char| char as u8)
        .chain([17, 31, 73, 47, 23u8].into_iter());
    let mut current_position = 0;
    let mut buffer = (0..buffer_length).collect::<Vec<_>>();
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in lengths.clone() {
            let mut temp_buffer = buffer.iter().cycle();

            // replace with advance_by
            for _ in 0..current_position {
                temp_buffer.next();
            }

            let buffer_copy = temp_buffer
                .take(length as usize)
                .cloned()
                .collect::<Vec<i32>>();

            for num in buffer_copy.iter().rev() {
                buffer[current_position as usize % buffer_length as usize] = *num;
                current_position += 1;
            }
            current_position += skip_size;
            skip_size += 1;
        }
    }

    Ok(buffer
        .chunks(16)
        .map(|chunk| {
            format!(
                "{:0>2x}",
                chunk.iter().cloned().reduce(|a, b| a ^ b).unwrap()
            )
        })
        .collect::<String>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("3,4,1,5", 5).unwrap(), 12)
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            part2("1,2,3", 256).unwrap(),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        )
    }
}

fn main() {
    let input = "14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244";

    println!("part1: {:?}", part1(input, 256));
    println!("part2: {:?}", part2(input, 256));
}
