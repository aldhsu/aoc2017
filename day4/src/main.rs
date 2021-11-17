fn main() {
    let input = include_str!("../input.txt");
    let part1 = input.lines().filter(|line| {
        let total_words = line.split(' ').count();
        let set = line.split(' ').collect::<std::collections::HashSet<_>>();
        total_words == set.iter().count()
    }).count();

    let part2 = input.lines().filter(|line| {
        let total_words = line.split(' ').count();
        let set = line
            .split(' ')
            .map(|span| {
                let mut vec: Vec<_> = span.chars().collect();
                vec.sort();
                vec
            })
            .collect::<std::collections::HashSet<_>>();
        total_words == set.iter().count()
    }).count();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
