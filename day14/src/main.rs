mod knot_hash;
use knot_hash::knot_hash;
use rayon::prelude::*;

fn part1(input: &str) -> u32 {
    (0..128)
        .into_par_iter()
        .map(|i| {
            knot_hash(&format!("{}-{}", input, i))
                .iter()
                .map(|byte| byte.count_ones())
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn hash_as_vec_bool(input: &str, i: i32) -> Vec<bool> {
    knot_hash(&format!("{}-{}", input, i)).into_iter().flat_map(|mut num| {
        let mask = 1;
        let mut result = vec![false; 8];

        for i in 0..8 {
            result[7 - i] = mask & num == 1;
            num >>= 1;
        }

        result
    }).collect()
}

fn part2(input: &str) -> u32 {
    let map : Vec<Vec<bool>> = (0..128)
        .into_iter()
        .map(|i| {
            hash_as_vec_bool(input, i)
        }).collect();

    group_cells(&map)
}

const DIRECTION_OFFSET: [(isize, isize); 4] = [
    (0, -1),
(-1, 0), (1, 0),
    (0, 1)
];

fn get_cell(x: usize, y: usize, map: &[Vec<bool>]) -> Option<&bool> {
    map.get(y)?.get(x)
}

fn group_cells(map: &[Vec<bool>]) -> u32 {
    let mut seen = std::collections::HashSet::new();
    let mut groups_count = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if !seen.insert((x,y)) { continue}
            if !cell { continue }

            groups_count += 1;

            let mut local_seen = std::collections::HashSet::new();
            let mut candidates_list: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();
            candidates_list.push_front((x, y));

            while let Some((x,y)) = candidates_list.pop_back() {
                if !local_seen.insert((x,y)) { continue}
                if let Some(true) = get_cell(x, y, map) {
                    DIRECTION_OFFSET.iter().filter_map(|(offset_x, offset_y)| {
                        let x = x as isize + offset_x;
                        let y = y as isize + offset_y;
                        (x.is_positive()  || y.is_positive()).then(|| {
                            (x as usize, y as usize)
                        })
                    }).for_each(|coord| candidates_list.push_front(coord));
                } else { continue};
            }

            seen.extend(local_seen)
        }
    }

    groups_count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(part1("flqrgnkx"), 8108)
    }

    // #[test]
    // fn into_vec_bool() {
    //     assert_eq!(hash_as_vec_bool("flqrgnkx", 0), vec![]);
    // }
    #[test]
    fn part2_works() {
        assert_eq!(part2("flqrgnkx"), 1242)
    }
}

fn main() {
    let input = "jzgqcdpd";

    println!("part1: {}", part1(input));
    println!("part1: {}", part2(input));
}
