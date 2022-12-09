use crate::problem::Problem;
use itertools::Itertools;

pub struct DayThree {}

fn common_items(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter().copied().filter(|a| y.contains(a)).collect()
}

fn item_value(item: u8) -> usize {
    let alphabet = (b'a'..=b'z').chain(b'A'..=b'Z').collect::<Vec<u8>>();
    alphabet.iter().position(|abc| *abc == item).unwrap() + 1_usize
}

impl Problem for DayThree {
    fn part_one(&self) -> String {
        let lines = include_str!("../data/day3.input")
            .split('\n')
            .filter_map(|l| match l {
                "" => None,
                _ => Some(l.as_bytes()),
            })
            .map(|l| common_items(&l[..l.len() / 2], &l[l.len() / 2..]))
            .map(|i| item_value(i[0]))
            .sum::<usize>();

        format!("Day 3 part 1: {}", lines)
    }
    fn part_two(&self) -> String {
        let lines = include_str!("../data/day3.input")
            .split('\n')
            .filter_map(|l| match l {
                "" => None,
                _ => Some(l.as_bytes()),
            })
            .tuples()
            .map(|(a, b, c)| common_items(a, &common_items(b, c)))
            .map(|i| item_value(i[0]))
            .sum::<usize>();
        format!("Day 3 part 2: {}", lines)
    }
}
