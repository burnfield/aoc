use crate::problem::Problem;
use std::{collections::HashMap, path::PathBuf};

pub struct DaySeven {}

fn parse() -> HashMap<PathBuf, usize> {
    let mut cd = vec!["/".to_string()];
    let mut sizes = HashMap::new();

    include_str!("../data/day7.input")
        .lines()
        .filter(|l| !l.starts_with("$ ls"))
        .filter(|l| !l.starts_with("dir"))
        .for_each(|l| match l.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                cd = vec!["/".to_string()];
            }
            ["$", "cd", ".."] => {
                cd.pop().unwrap();
            }
            ["$", "cd", name] => {
                cd.push(name.to_string());
            }
            [size, _name] => {
                let size: usize = size.parse().unwrap();
                for i in 0..cd.len() {
                    let path = PathBuf::from_iter(cd[0..=i].iter());
                    *sizes.entry(path).or_insert(0_usize) += size;
                }
            }
            _ => {}
        });
    sizes
}

impl Problem for DaySeven {
    fn part_one(&self) -> String {
        let sizes = parse();

        let res: usize = sizes.into_values().filter(|x| *x <= 100_000).sum();

        format!("Day 7 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let sizes = parse();

        let size_required: usize = sizes.values().max().unwrap() - 40_000_000;
        let res: usize = sizes
            .into_values()
            .filter_map(|x| x.checked_sub(size_required))
            .min()
            .map(|x| x + size_required)
            .unwrap();

        format!("Day 7 part 2: {}", res)
    }
}
