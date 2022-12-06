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
