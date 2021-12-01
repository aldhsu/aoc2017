const BUFFER_LENGTH: u8 = 255;
pub fn knot_hash(input: &str) -> Vec<u8> {
    let lengths = input
        .chars()
        .map(|char| char as u8)
        .chain([17, 31, 73, 47, 23u8].into_iter())
        .collect::<Vec<_>>();
    let mut current_position = 0usize;
    let mut buffer = (0..=BUFFER_LENGTH).collect::<Vec<_>>();
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in &lengths {
            reverse_section(&mut buffer, current_position, *length as usize);

            current_position += *length as usize;
            current_position += skip_size;
            skip_size += 1;
        }
    }

    buffer
        .chunks(16)
        .map(|chunk| chunk.iter().cloned().reduce(|a, b| a ^ b).unwrap())
        .collect()
}

fn reverse_section<T>(buffer: &mut Vec<T>, start: usize, length: usize) {
    let len = buffer.len();
    for i in 0..(length / 2) {
        let left = (start + i) % len;
        let right = (start + length - 1 - i) % len;
        buffer.swap(left, right);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knot_hash_works() {
        assert_eq!(
            knot_hash("1,2,3")
                .iter()
                .map(|chunk| { format!("{:0>2x}", chunk) })
                .collect::<String>(),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        )
    }

    #[test]
    fn reverse_section_works() {
        let mut buffer = vec![0, 1, 2, 3, 4, 5];
        reverse_section(&mut buffer, 1, 3);
        assert_eq!(buffer, vec![0, 3, 2, 1, 4, 5]);
    }
}
