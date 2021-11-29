#[derive(Debug)]
struct Layer {
    depth: usize,
    range: usize,
    cost: usize,
}

fn parse_input(input: &str) -> Vec<Layer> {
    input.lines().map(|line| {
        let (depth, range) = line.split_once(": ").unwrap();
        let depth: usize = depth.parse().unwrap();
        let range = range.parse().unwrap();

        Layer {
            depth,
            range,
            cost: depth * range,
        }
    }).collect()
}

fn part1(input: &str) -> usize {
    let layers = parse_input(input);
    layers.iter().map(|layer| {
        if layer.depth % (layer.range + layer.range - 2) == 0 {
            layer.cost
        } else {
            0
        }
    }).sum()
}

fn part2(input: &str) -> usize {
    let layers = parse_input(input);

    (0..).find(|pause| {
        !layers.iter().any(|layer| {
            (layer.depth + pause) % (layer.range + layer.range - 2) == 0 
        })
    }).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn it_works_part1() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(part1(input), 24)
    }

    #[test]
    fn it_works_part2() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(part2(input), 10)
    }
}
fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
