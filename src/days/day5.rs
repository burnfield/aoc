use crate::problem::Problem;
use itertools::Itertools;
use std::collections::BTreeMap;

pub struct DayFive {}

struct Order {
    num: usize,
    from: usize,
    to: usize,
}

impl Order {
    fn new((num, from, to): (usize, usize, usize)) -> Self {
        Self { num, from, to }
    }
}

fn parse() -> (BTreeMap<usize, Vec<char>>, Vec<Order>) {
    include_str!("../data/day5.input")
        .split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .map(|(s, o)| (parse_stacks(s), parse_orders(o)))
        .unwrap()
}
fn parse_stacks(input: &str) -> BTreeMap<usize, Vec<char>> {
    // could make it larger but faster with last iterator
    let num_stacks = input
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .max()
        .unwrap();

    let mut stacks: BTreeMap<usize, Vec<char>> = BTreeMap::new();
    input
        .chars()
        .enumerate()
        .filter(|(_i, x)| ('A'..='Z').contains(x))
        .for_each(|(i, c)| {
            let i = (3 + i) / 4;
            let m_i = i % num_stacks;
            let s = if m_i == 0 { num_stacks } else { m_i };
            stacks.entry(s).or_insert_with(Vec::new).insert(0, c);
        });
    stacks
}

fn parse_orders(input: &str) -> Vec<Order> {
    input
        .split(['\n', ' '])
        .filter_map(|x| x.parse::<usize>().ok())
        .tuples::<(usize, usize, usize)>()
        .map(Order::new)
        .collect::<Vec<Order>>()
}

impl Problem for DayFive {
    fn part_one(&self) -> String {
        let (mut stacks, orders) = parse();

        orders.iter().for_each(|Order { num, from, to }| {
            for _i in 0..*num {
                if let Some(x) = stacks.get_mut(from).unwrap().pop() {
                    stacks.get_mut(to).unwrap().push(x)
                }
            }
        });

        let res = stacks.values().filter_map(|v| v.last()).collect::<String>();

        assert!(res == "VPCDMSLWJ");

        format!("Day 3 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let (mut stacks, orders) = parse();

        orders.iter().for_each(|Order { num, from, to }| {
            let cur_len = stacks.get(to).unwrap().len();
            for _i in 0..*num {
                if let Some(x) = stacks.get_mut(from).unwrap().pop() {
                    stacks.get_mut(to).unwrap().insert(cur_len, x)
                }
            }
        });

        let res = stacks.values().filter_map(|v| v.last()).collect::<String>();

        assert!(res == "TPWCGNCCG");

        format!("Day 3 part 2: {}", res)
    }
}
