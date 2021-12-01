fn gen_a(previous: usize) -> usize {
    (previous * 16807) % 2147483647
}

fn gen_b(previous: usize) -> usize {
    (previous * 48271) % 2147483647
}

const MASK: usize = 0b1111_1111_1111_1111;

fn part1(mut a: usize , mut b: usize) -> usize {
    (0..40_000_000)
        .filter(|_| {
            a = gen_a(a);
            b = gen_b(b);
            let a_bits = a & MASK;
            let b_bits = b & MASK;

            a_bits == b_bits
        }).count()
}

use std::thread::spawn;
use std::sync::mpsc::{self, Sender};

fn part2(a: usize, b: usize) -> usize {
    let (tx_a, rx_a) = mpsc::channel();
    let (tx_b, rx_b) = mpsc::channel();

    fn generator(tx: Sender<usize>, starting_value: usize, function: fn(usize) -> usize, modulo: usize) -> impl Fn() {
        move || {
            let mut starting_value = starting_value;
            let mut count = 0;
            while count < 5_000_000 {
                starting_value = (function)(starting_value);
                if starting_value % modulo == 0 {
                    tx.send(starting_value).expect("couldn't send");
                    count += 1;
                }
            }
        }
    }

    spawn(generator(tx_a, a, gen_a, 4));
    spawn(generator(tx_b, b, gen_b, 8));

    let mut count = 0; 
    while let (Ok(a), Ok(b)) = (rx_a.recv(), rx_b.recv()) {
        let a_bits = a & MASK;
        let b_bits = b & MASK;

        if a_bits == b_bits {
            count += 1;
        };
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let part1 = part1(65, 8921);
        assert_eq!(part1, 588)
    }

    #[test]
    fn it_works_part2() {
        let part2 = part2(65, 8921);
        assert_eq!(part2, 309)
    }
}

fn main() {
    let part1 = part1(679, 771);
    let part2 = part2(679, 771);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
