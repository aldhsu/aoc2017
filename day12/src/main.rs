use std::collections::VecDeque;

fn parse_input(input: &str) -> std::collections::HashMap<i32, Vec<i32>> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.trim().split_once("<->").expect("couldn't parse line");
            let right = right
                .split(", ")
                .map(|num| {
                    num.trim()
                        .parse::<i32>()
                        .unwrap_or_else(|_| panic!("couldn't parse right {:?}", num))
                })
                .collect::<Vec<i32>>();

            (
                left.trim()
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("couldn't parse left {}", left)),
                right,
            )
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let mut items = VecDeque::new();
    items.push_front(0i32);

    let mut seen = std::collections::HashSet::new();

    while let Some(item) = items.pop_back() {
        if seen.insert(item) {
            if let Some(values) = map.get(&item) {
                for value in values {
                    items.push_front(*value)
                }
            }
        }
    }

    seen.len()
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);

    let mut seen = std::collections::HashSet::new();
    let mut groups: Vec<std::collections::HashSet<i32>> = vec![];

    for k in map.keys() {
        if seen.contains(k) {
            continue;
        }
        let mut local_group: std::collections::HashSet<i32> = std::collections::HashSet::new();

        let mut items = VecDeque::new();
        items.push_front(k);

        while let Some(item) = items.pop_back() {
            if seen.insert(item) && local_group.insert(*item) {
                if let Some(values) = map.get(item) {
                    for value in values {
                        items.push_front(&*value)
                    }
                }
            } else {
                if local_group.contains(item) {
                    continue;
                }
                let group = groups.swap_remove(
                    groups
                        .iter()
                        .position(|group| group.contains(item))
                        .expect("should have group"),
                );
                local_group.extend(group.into_iter());
            }
        }
        groups.push(local_group)
    }

    groups.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = part1(input);
    println!("part1: {}", part1);

    let part2 = part2(input);
    println!("part1: {}", part2);
}
