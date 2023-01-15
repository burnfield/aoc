use crate::problem::Problem;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};

pub struct DayFifteen {}

impl Problem for DayFifteen {
    fn part_one(&self) -> String {
        let target_y = 2000000;
        let res = include_str!("../data/day15.input")
            .lines()
            .map(line_to_ints)
            .filter_map(|(x0, y0, x1, y1)| {
                let manhatan_dist = (x0 - x1).abs() + (y0 - y1).abs();
                if ((y0 - manhatan_dist)..=(y0 + manhatan_dist)).contains(&target_y) {
                    Some((x0, y0, x1, y1, manhatan_dist))
                } else {
                    None
                }
            })
            .flat_map(|(x0, y0, x1, y1, manhatan_dist)| -> HashSet<i64> {
                let sides = manhatan_dist - (y0 - target_y).abs();
                ((x0 - sides)..=(x0 + sides))
                    .filter_map(|x| {
                        if (y0 == target_y && x == x0) || (y1 == target_y && x == x1) {
                            None
                        } else {
                            Some(x)
                        }
                    })
                    .collect()
            })
            .collect::<HashSet<i64>>()
            .len();
        format!("Day 15 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let mut dec_lines = HashSet::new();
        let mut inc_lines = HashSet::new();
        let mut sensors = HashMap::new();
        include_str!("../data/day15.input")
            .lines()
            .map(line_to_ints)
            .for_each(|(x0, y0, x1, y1)| {
                let manhattan = (x0 - x1).abs() + (y0 - y1).abs();
                *sensors.entry((x0, y0)).or_insert(manhattan) = manhattan;
                // Exclusive lines y = kx + m, k = (+-)1, stores m in hashset
                inc_lines.insert(y0 - manhattan - 1 - x0);
                dec_lines.insert(y0 - manhattan - 1 + x0);
                inc_lines.insert(y0 + manhattan + 1 - x0);
                dec_lines.insert(y0 + manhattan + 1 + x0);
            });

        let limit = 4_000_000;
        let intersections: HashSet<(i64, i64)> =
            HashSet::from_iter(inc_lines.iter().flat_map(|i_m| {
                dec_lines
                    .iter()
                    .filter_map(|d_m| {
                        let x = (d_m - i_m) / 2;
                        let y = x + i_m;
                        if x < 0 || x > limit || y < 0 || y > limit {
                            None
                        } else {
                            Some((x, y))
                        }
                    })
                    .collect::<HashSet<(i64, i64)>>()
            }));

        let res = intersections
            .iter()
            .find(|(x, y)| is_outside_all_sensors(*x, *y, &sensors))
            .map(|(x, y)| x * 4_000_000 + y)
            .unwrap();

        format!("Day 15 part 2: {}", res)
    }
}

fn is_outside_all_sensors(x: i64, y: i64, sensors: &HashMap<(i64, i64), i64>) -> bool {
    for ((x0, y0), sensor_range) in sensors.iter() {
        let manhattan = (x0 - x).abs() + (y0 - y).abs();
        if sensor_range >= &manhattan {
            return false;
        }
    }
    true
}

fn line_to_ints(l: &str) -> (i64, i64, i64, i64) {
    scan_fmt!(
        l,
        "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
        i64,
        i64,
        i64,
        i64
    )
    .unwrap()
}
