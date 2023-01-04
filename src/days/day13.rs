use crate::problem::Problem;
use core::cmp::Ordering;
use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit0, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};
use std::fmt;

pub struct DayThirteen {}

#[derive(Debug, Eq, Clone)]
enum Packet {
    Value(i32),
    List(Vec<Packet>),
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Value(x) => write!(f, "{}", x),
            Packet::List(y) => write!(f, "[{}]", y.iter().map(|y| y.to_string()).join(",")),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(x), Packet::Value(y)) => x.cmp(y),
            (Packet::Value(x), Packet::List(y)) => vec![Packet::Value(*x)].cmp(y),
            (Packet::List(x), Packet::Value(y)) => x.cmp(&vec![Packet::Value(*y)]),
            (Packet::List(x), Packet::List(y)) => x.cmp(y),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Value(x), Packet::Value(y)) => x == y,
            (Packet::Value(x), Packet::List(y)) => &vec![Packet::Value(*x)] == y,
            (Packet::List(x), Packet::Value(y)) => x == &vec![Packet::Value(*y)],
            (Packet::List(x), Packet::List(y)) => {
                if !x.is_empty() && y.is_empty() {
                    false
                } else {
                    x.iter()
                        .zip(y.iter())
                        .fold(true, |acc, (x, y)| acc && (x == y))
                }
            }
        }
    }
}

fn parse_value(s: &str) -> IResult<&str, Packet> {
    map(digit0, |x: &str| Packet::Value(x.parse::<i32>().unwrap()))(s)
}

fn parse_values(s: &str) -> IResult<&str, Vec<Packet>> {
    if s.starts_with(']') {
        Ok((s, vec![]))
    } else {
        separated_list0(tag(","), alt((parse_packet, parse_value)))(s)
    }
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    map(delimited(tag("["), parse_values, tag("]")), Packet::List)(s)
}

impl Problem for DayThirteen {
    fn part_one(&self) -> String {
        let pairs: Vec<(Packet, Packet)> = include_str!("../data/day13.input")
            .split("\n\n")
            .filter_map(|p| {
                p.split('\n')
                    .filter_map(|l| parse_packet(l).ok())
                    .map(|x| x.1)
                    .collect_tuple()
            })
            .collect();

        let corr: usize = pairs
            .iter()
            .enumerate()
            .filter_map(|(x, (left, right))| if left < right { Some(x + 1) } else { None })
            .sum();

        format!("Day 13 part 1: {}", corr)
    }

    fn part_two(&self) -> String {
        let mut pairs: Vec<Packet> = include_str!("../data/day13.input")
            .lines()
            .filter(|l| !l.is_empty())
            .filter_map(|l| parse_packet(l).ok())
            .map(|l| l.1)
            .collect::<Vec<Packet>>();

        pairs.extend(vec![
            Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
            Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
        ]);
        pairs.sort();

        let f = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
        let s = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);

        let first = pairs.iter().position(|l| &f == l).unwrap() + 1;
        let second = pairs.into_iter().position(|l| s == l).unwrap() + 1;

        format!("Day 13 part 2: {}", first * second)
    }
}

#[test]
fn test_parse_values() {
    assert_eq!(
        parse_values("1,2,3").unwrap().1,
        vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)]
    );
}

#[test]
fn test_parse_value() {
    assert_eq!(parse_value("1").unwrap().1, Packet::Value(1));
}

#[test]
fn test_parse_packet() {
    assert_eq!(
        parse_packet("[1,2,3]").unwrap().1,
        Packet::List(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)])
    );
    assert_eq!(parse_packet("[]").unwrap().1, Packet::List(vec![]));
    assert_eq!(
        parse_packet("[[]]").unwrap().1,
        Packet::List(vec![Packet::List(vec![])])
    );
    assert_eq!(
        parse_packet("[1]").unwrap().1,
        Packet::List(vec![Packet::Value(1)])
    );
    assert_eq!(
        parse_packet("[1,[2]]").unwrap().1,
        Packet::List(vec![Packet::Value(1), Packet::List(vec![Packet::Value(2)])])
    );
    assert_eq!(
        parse_packet("[[1],[2,3]]").unwrap().1,
        Packet::List(vec![
            Packet::List(vec![Packet::Value(1)]),
            Packet::List(vec![Packet::Value(2), Packet::Value(3)])
        ])
    );
    assert_eq!(
        parse_packet("[[1],2,3]").unwrap().1,
        Packet::List(vec![
            Packet::List(vec![Packet::Value(1)]),
            Packet::Value(2),
            Packet::Value(3)
        ])
    );
}

#[test]
fn test_ord() {
    assert_ne!(Packet::Value(1), Packet::Value(2));
    assert_eq!(Packet::List(vec![Packet::Value(2)]), Packet::Value(2));
    assert!(Packet::List(vec![Packet::Value(2), Packet::Value(3)]) < Packet::Value(3));
    assert!(Packet::List(vec![Packet::List(vec![])]) > Packet::List(vec![]));
    assert!(
        Packet::List(vec![Packet::Value(9)])
            > Packet::List(vec![Packet::Value(8), Packet::Value(7)])
    );
    assert!(
        Packet::List(vec![Packet::List(vec![Packet::Value(4)]), Packet::Value(4)])
            < Packet::List(vec![
                Packet::List(vec![Packet::Value(4)]),
                Packet::Value(4),
                Packet::Value(4)
            ])
    );
    assert!(
        Packet::List(vec![Packet::Value(4), Packet::Value(4), Packet::Value(4)])
            > Packet::List(vec![Packet::Value(4), Packet::Value(4)])
    );
    assert_ne!(
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
        Packet::List(vec![])
    );
}
