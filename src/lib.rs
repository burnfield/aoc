use itertools::Itertools;

pub fn day01() {
    let res: Vec<usize> = include_str!("day1.input")
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|food| food.parse::<usize>().unwrap_or(0))
                .sum()
        })
        .sorted()
        .rev()
        .collect();

    println!("Day 1 part 1: {}", res[0]);
    println!("Day 1 part 2: {}", res.iter().take(3).sum::<usize>());
}

#[test]
fn test1() {
    day01();
}

fn day02_part1((a, x): (usize, usize)) -> usize {
    if a == x {
        x + 3
    } else if x == (1 + a % 3) {
        x + 6
    } else {
        x
    }
}

fn day02_part2((a, x): (usize, usize)) -> usize {
    let loose_value = if a - 1 == 0 { 3 } else { a - 1 };
    let win_value = if a + 1 == 4 { 1 } else { a + 1 };
    match x {
        1 => loose_value,
        2 => 3 + a,
        3 => 6 + win_value,
        _ => panic!(),
    }
}

fn day02() {
    let parts: Vec<&dyn Fn((usize, usize)) -> usize> = vec![&day02_part1, &day02_part2];
    for (idx, part) in parts.iter().enumerate() {
        let res: usize = include_str!("day2.input")
            .split("\n")
            .map(|gestures| -> usize {
                gestures
                    .split(" ")
                    .filter_map(|play| match "ABCXYZ".find(play) {
                        Some(play) => Some(1 + play % 3),
                        _ => None,
                    })
                    .tuples()
                    .map(part)
                    .sum::<usize>()
            })
            .sum();

        println!("Day 2 part {}: {}", idx + 1, res);
    }
}

#[test]
fn test2() {
    day02();
}
