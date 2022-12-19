use crate::problem::Problem;
use itertools::Itertools;

pub struct DayEleven {}

#[derive(Debug, Clone)]
struct Monkey {
    inspections: usize,
    items: Vec<usize>,
    operation: Operations,
    div: usize,
    false_monkey: usize,
    true_monkey: usize,
}

impl Monkey {
    fn new(input: &str) -> Self {
        let mut input = input.lines();

        // TODO: fix this clusterfuck
        // I hate this day..
        input.next();
        let items: Vec<usize> = input
            .next()
            .map(|l| {
                l.trim()
                    .trim_start_matches("Starting items: ")
                    .split(", ")
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .unwrap();

        let operation: Operations = input
            .next()
            .map(|l| l.split_whitespace().collect::<Vec<_>>())
            .map(|l| match l[..] {
                ["Operation:", .., "=", opa, op, opb] => {
                    if opa == opb {
                        Operations::Square()
                    } else if op == "*" {
                        Operations::Multiply(opb.parse::<usize>().unwrap())
                    } else {
                        assert!(op == "+");
                        Operations::Add(opb.parse::<usize>().unwrap())
                    }
                }
                _ => unreachable!(),
            })
            .unwrap();

        let div: usize = *input
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<usize>>()
            .first()
            .unwrap();

        let true_monkey = input
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let false_monkey = input
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let inspections = 0;

        Monkey {
            inspections,
            items,
            operation,
            div,
            true_monkey,
            false_monkey,
        }
    }
}

#[derive(Debug, Clone)]
enum Operations {
    Add(usize),
    Multiply(usize),
    Square(),
}

fn solution(num_times: usize, panic_pill: usize) -> Vec<usize> {
    let mut apor: Vec<Monkey> = include_str!("../data/day11.input")
        .split("\n\n")
        .map(Monkey::new)
        .collect();
    let common_div: usize = apor.iter().map(|m| m.div).product();
    for round in 0..(num_times * apor.len()) {
        let ap_idx = round % apor.len();
        while !apor[ap_idx].items.is_empty() {
            apor[ap_idx].inspections += 1;
            let item = apor[ap_idx].items.pop().unwrap();
            let item = item % common_div;
            let item = match apor[ap_idx].operation {
                Operations::Add(x) => (item + x) / panic_pill,
                Operations::Multiply(x) => item * x / panic_pill,
                Operations::Square() => item * item / panic_pill,
            };
            let next_ap = if item % apor[ap_idx].div == 0 {
                apor[ap_idx].true_monkey
            } else {
                apor[ap_idx].false_monkey
            };
            apor[next_ap].items.push(item);
        }
    }
    let res = apor
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .collect::<Vec<usize>>();
    res
}

impl Problem for DayEleven {
    fn part_one(&self) -> String {
        let res = solution(20, 3);

        let res = res[0] * res[1];
        format!("Day 7 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let res = solution(10000, 1);

        let res = res[0] * res[1];
        format!("Day 7 part 2: {}", res)
    }
}
