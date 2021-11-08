use std::{fs, io::Read};

fn main() {
    let input = include_str!("input.txt");
    let checksum: u64 = input
        .lines()
        .map(|line| {
            let nums = line
                .split("	")
                .map(|num| num.parse::<u64>().expect("can't parse"))
                .collect::<Vec<_>>();
            let min = nums.iter().min().expect("can't find min");
            let max = nums.iter().max().expect("can't find max");
            max - min
        })
        .sum();

    println!("part1: {}", checksum);

    let checksum: u64 = input
        .lines()
        .map(|line| {
            let nums = line
                .split("	")
                .map(|num| num.parse::<u64>().expect("can't parse"))
                .collect::<Vec<_>>();

            nums.iter()
                .enumerate()
                .find_map(|(i, num)| {
                    nums[(i + 1)..].iter().find_map(|other| {
                        let (big, little) = if other > num { (other, num) } else { (num, other) };
                        if (*big % *little) == 0 {
                            Some(big / little)
                        } else {
                            None
                        }
                    })
                }).expect(&format!("couldn't find {}", line))
        })
        .sum();

    println!("part2: {}", checksum);
}
