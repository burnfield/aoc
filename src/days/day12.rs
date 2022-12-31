use crate::problem::Problem;
use std::collections::{HashMap, HashSet, VecDeque};

type Node = (i32, i32);

pub struct DayTwelve {}

fn neighbors(current_node: &Node, nodes: &HashMap<Node, &u8>) -> Vec<Node> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let current_height = nodes.get(current_node).unwrap();
    directions
        .iter()
        .map(|direction| (current_node.0 + direction.0, current_node.1 + direction.1))
        .filter_map(|next_node| nodes.get_key_value(&next_node))
        .filter_map(|(next_node, next_height)| {
            let diff = current_height.wrapping_sub(**next_height);
            if diff == u8::MAX
                || diff <= 28
                || **current_height == b'S'
                || (**current_height == b'z' && **next_height == b'E')
            {
                Some(*next_node)
            } else {
                None
            }
        })
        .collect::<Vec<Node>>()
}

fn solution(starting_height: &[u8]) -> usize {
    let height_map: HashMap<Node, &u8> = HashMap::from_iter(
        include_bytes!("../data/day12.input")
            .split(|b| *b == b'\n')
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .map(|(x, c)| ((x as i32, y as i32), c))
                    .collect::<Vec<(Node, &u8)>>()
            }),
    );

    let end = height_map
        .iter()
        .find_map(|(n, c)| if **c == b'E' { Some(n) } else { None })
        .unwrap();

    let mut visited = HashSet::new();

    let mut to_visit: VecDeque<(Node, usize)> = height_map
        .iter()
        .filter_map(|(node, height)| {
            if starting_height.contains(height) {
                Some((*node, 0))
            } else {
                None
            }
        })
        .collect();

    while let Some((node, distance)) = to_visit.pop_front() {
        if !visited.insert(node) {
            continue;
        }
        if node == *end {
            return distance;
        }
        for neighbor in neighbors(&node, &height_map) {
            to_visit.push_back((neighbor, distance + 1));
        }
    }
    unreachable!();
}

impl Problem for DayTwelve {
    fn part_one(&self) -> String {
        let res = solution(&[b'S']);

        format!("Day 12 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let res = solution(&[b'S', b'a']);

        format!("Day 7 part 2: {}", res)
    }
}
