use crate::problem::Problem;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Add;

pub struct DayFourteen {}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

type RockPath = HashSet<Point>;

const DIRECTIONS: [Point; 3] = [
    Point { x: 0, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: 1, y: 1 },
];

const SOURCE: Point = Point { x: 500, y: 0 };

fn tick(grain: Point, xmin: i32, xmax: i32, ymax: i32, scan: &mut RockPath) -> bool {
    if grain.x > xmax || grain.x < xmin || grain.y > ymax {
        return true;
    }

    for direction in DIRECTIONS {
        let next = grain + direction;
        if scan.contains(&next) {
            continue;
        }
        if tick(next, xmin, xmax, ymax, scan) {
            return true;
        }
    }

    scan.insert(grain);
    false
}

impl Problem for DayFourteen {
    fn part_one(&self) -> String {
        let mut scan = parse_input();
        let (xmin, xmax, ymax) = part_one_edges(&scan);

        let before = scan.len();
        tick(SOURCE, xmin, xmax, ymax, &mut scan);
        format!("Day 14 part 1: {}", scan.len() - before)
    }

    fn part_two(&self) -> String {
        let mut scan = parse_input();
        let (xmin, xmax, ymax) = part_two_edges(&mut scan);

        // floor cant be wider than triangle (+1 for not falling over)
        let (xmin, xmax, ymax) = (xmin - 3, xmax + 3, ymax + 2);
        for x in xmin..xmax {
            scan.insert(Point { x, y: ymax });
        }

        let before = scan.len();
        tick(SOURCE, xmin, xmax, ymax + 1, &mut scan);
        format!("Day 14 part 2: {}", scan.len() - before)
    }
}

fn part_one_edges(scan: &HashSet<Point>) -> (i32, i32, i32) {
    scan.iter()
        .fold((i32::MAX, 0, 0), |(mut xmin, mut xmax, mut ymax), curr| {
            if curr.x > xmax {
                xmax = curr.x;
            } else if curr.x < xmin {
                xmin = curr.x;
            }
            if curr.y > ymax {
                ymax = curr.y;
            }
            (xmin, xmax, ymax)
        })
}

fn part_two_edges(scan: &mut HashSet<Point>) -> (i32, i32, i32) {
    scan.iter()
        .fold((i32::MAX, 0, 0), |(xmin, xmax, ymax), curr| {
            if curr.y > ymax {
                (SOURCE.x - curr.y, SOURCE.x + curr.y, curr.y)
            } else {
                (xmin, xmax, ymax)
            }
        })
}

fn parse_input() -> HashSet<Point> {
    let mut scan = RockPath::new();
    include_str!("../data/day14.input").lines().for_each(|l| {
        l.split(" -> ")
            .map(Point::parse_from_scanner)
            .tuple_windows()
            .for_each(|(start, end)| {
                let mut xrange = [start.x, end.x];
                xrange.sort();
                let mut yrange = [start.y, end.y];
                yrange.sort();
                for x in xrange[0]..=xrange[1] {
                    for y in yrange[0]..=yrange[1] {
                        scan.insert(Point { x, y });
                    }
                }
            });
    });
    scan
}

impl Point {
    fn parse_from_scanner(input: &str) -> Point {
        let input: Vec<&str> = input.split(',').collect();
        match input[..] {
            [x, y] => Point {
                x: x.parse::<i32>().unwrap(),
                y: y.parse::<i32>().unwrap(),
            },
            _ => unreachable!(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
