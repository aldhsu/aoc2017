use itertools::Itertools;

struct Dag;

type NodeMap = std::collections::HashMap<String, Node>;
type RecursiveResult = Result<i32, (i32, Vec<i32>)>;

impl Dag {
    fn root(graph: &Graph) -> String {
        let Graph { nodes } = graph;
        let mut candidate_roots: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for (key, leaves) in nodes {
            *candidate_roots.entry(key.clone()).or_insert(0) += 1;

            if let Some(leaves) = &leaves.nodes {
                for leaf in leaves {
                    *candidate_roots.entry(leaf.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut roots = candidate_roots
            .iter()
            .filter_map(|(k, v)| (v == &1).then(|| k));
        assert!(roots.clone().count() == 1);
        roots.next().expect("no candidates").to_string()
    }


    fn build_dag(graph: Graph) -> RecursiveResult {
        let root = Self::root(&graph);
        let Graph { nodes } = graph;

        fn sum_vals(node: &Node, map: &NodeMap) -> RecursiveResult {
            if let Some(nodes) = &node.nodes {
                let children = nodes
                    .iter()
                    .map(|node| {
                        let node = &map[node];
                        sum_vals(node, map).map(|num| (num, node.num))
                    })
                    .collect::<Result<Vec<(i32, i32)>, (i32, Vec<i32>)>>()?;

                if children.iter().map(|(value, _)| value).unique().count() > 1 {
                    let mut counts =  std::collections::HashMap::new();
                    for (value, _) in children.iter() {
                        *counts.entry(value).or_insert(0) += 1;
                    }

                    let wrong = counts.iter().find_map(|(k, count)| (count == &1).then(|| k)).expect("can't find wrong");
                    let right = counts.iter().find_map(|(k, count)| (count == &2).then(|| k)).expect("can't find right");
                    let diff = *wrong - *right;

                    let val = children.iter().find_map(|(value, num)| (&value == wrong).then(|| num)).expect("can't find val");


                    Err((*val - diff, children.into_iter().map(|(value, _)| value).collect()))
                } else {
                    Ok(node.num + children.iter().map(|(count, _)| count).sum::<i32>())
                }
            } else {
                Ok(node.num)
            }
        }

        let root = &nodes[&root];
        sum_vals(root, &nodes)
    }
}

struct Graph {
    nodes: std::collections::HashMap<String, Node>,
}

#[derive(Debug)]
enum Error {
    NodeParseError(&'static str),
}

#[derive(Debug)]
struct Node {
    name: String,
    num: i32,
    nodes: Option<Vec<String>>,
}

impl TryFrom<&str> for Node {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (name, rest) = value
            .split_once(' ')
            .ok_or(Error::NodeParseError("no name"))?;
        // num needs to be processed
        let mut nodes = None;
        let num = if let Some((num, rest)) = rest.split_once(' ') {
            let (_, rest) = rest
                .split_once(' ')
                .ok_or(Error::NodeParseError("no arrow"))?;

            nodes = Some(
                rest.split(", ")
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>(),
            );
            num
        } else {
            rest
        };

        Ok(Self {
            num: num[1..num.len() - 1]
                .parse()
                .map_err(|_| Error::NodeParseError("can't parse"))?,
            name: name.to_string(),
            nodes,
        })
    }
}

impl TryFrom<&str> for Graph {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            nodes: input
                .lines()
                .map(|line| {
                    let node: Node = line.try_into()?;
                    Ok((node.name.clone(), node))
                })
                .collect::<Result<NodeMap, Error>>()?,
        })
    }
}
fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");
    let graph: Graph = input.try_into()?;

    let part1 = Dag::root(&graph);
    println!("part1 {}", part1);

    let part2 = Dag::build_dag(graph);
    dbg!(part2);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{Dag, Graph};

    #[test]
    fn it_works() {
        let input = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#;
        let graph: Graph = input.try_into().unwrap();

        let part1 = Dag::root(&graph);
        assert_eq!("tknk", part1);
    }

    #[test]
    fn it_finds_unbalanced() {
        let input = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#;
        let graph: Graph = input.try_into().unwrap();

        let part2 = Dag::build_dag(graph);
        dbg!(&part2);
        if let Err((n, _)) = part2 {
            assert_eq!(n, 60);
        } else {
            assert!(false)
        }
    }
}
