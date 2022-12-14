use crate::problem::Problem;
use std::collections::HashSet;

pub struct DayEight {}

fn parse() -> Vec<Vec<i8>> {
    Vec::from_iter(
        include_str!("../data/day8.input")
            .lines()
            .map(|l| l.chars().map(|x| x.to_digit(10).unwrap() as i8).collect()),
    )
}

fn find_max(max: &mut i8, d: &[Vec<i8>], x: usize, y: usize, res: &mut HashSet<(usize, usize)>) {
    if *max < d[y][x] {
        *max = d[y][x];
        res.insert((y, x));
    }
}

impl Problem for DayEight {
    fn part_one(&self) -> String {
        let d = parse();
        let mut res: HashSet<(usize, usize)> = HashSet::new();
        let l = d.len();
        for x in 0..l {
            let mut max = -1;
            for y in 0..l {
                find_max(&mut max, &d, y, x, &mut res);
                if max == 9 {
                    break;
                }
            }

            max = -1;
            for y in 0..l {
                find_max(&mut max, &d, x, y, &mut res);
                if max == 9 {
                    break;
                }
            }

            max = -1;
            for y in (0..l).rev() {
                find_max(&mut max, &d, y, x, &mut res);
                if max == 9 {
                    break;
                }
            }

            max = -1;
            for y in (0..l).rev() {
                find_max(&mut max, &d, x, y, &mut res);
                if max == 9 {
                    break;
                }
            }
        }

        assert!(res.len() == 1851);

        format!("Day 8 part 1: {}", res.len())
    }

    fn part_two(&self) -> String {
        let d = parse();
        let mut max = 0;
        let l = d.len();
        for x in 0..l {
            for y in 0..l {
                let current_height = d[x][y];
                let mut view_east = 0;
                for xe in (x..l).skip(1) {
                    view_east += 1;
                    if current_height <= d[xe][y] {
                        break;
                    }
                }
                let mut view_west = 0;
                for xw in (0..x).rev() {
                    view_west += 1;
                    if current_height <= d[xw][y] {
                        break;
                    }
                }
                let mut view_north = 0;
                for yn in (y..l).skip(1) {
                    view_north += 1;
                    if current_height <= d[x][yn] {
                        break;
                    }
                }
                let mut view_south = 0;
                for ys in (0..y).rev() {
                    view_south += 1;
                    if current_height <= d[x][ys] {
                        break;
                    }
                }
                let platform = view_east * view_west * view_north * view_south;
                if platform > max {
                    max = platform;
                }
            }
        }

        assert!(max == 574080);

        format!("Day 8 part 2: {}", max)
    }
}
