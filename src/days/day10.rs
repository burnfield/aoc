use crate::problem::Problem;

pub struct DayTen {}

enum Instr {
    Noop,
    Addx(i32),
}

fn pixel_sprite_overlap(pos: usize, sprite_pos: i32) -> bool {
    if sprite_pos.is_positive() {
        (7u64 << sprite_pos) & (1 << pos) != 0
    } else {
        (7u64 >> sprite_pos.abs()) & (1 << pos) != 0
    }
}

fn parse() -> Vec<Instr> {
    include_str!("../data/day10.input")
        .lines()
        .map(str::split_whitespace)
        .map(|l| l.collect::<Vec<&str>>())
        .map(|l| match l[..] {
            ["noop"] => Instr::Noop,
            ["addx", x] => Instr::Addx(x.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect::<Vec<Instr>>()
}

impl Problem for DayTen {
    fn part_one(&self) -> String {
        let inputs = parse();

        //TODO: remove this mut somehow??
        let mut x = 1;
        let res: i32 = inputs
            .iter()
            .flat_map(|i| match i {
                Instr::Noop => vec![x],
                Instr::Addx(val) => {
                    let res = vec![x; 2];
                    x += val;
                    res
                }
            })
            .enumerate()
            .map(|(i, x)| {
                let i: i32 = i.try_into().unwrap();
                x * (i + 1)
            })
            .skip(19)
            .step_by(40)
            .sum();

        assert!(res == 16480);

        format!("Day 7 part 1: {}", res)
    }

    fn part_two(&self) -> String {
        let inputs = parse();

        //TODO: remove this mut somehow??
        let mut sprite_pos = 0;
        let cpu_output = inputs
            .iter()
            .flat_map(|instr| match instr {
                Instr::Noop => vec![Instr::Noop],
                // simplify for enumerate
                Instr::Addx(x) => vec![Instr::Noop, Instr::Addx(*x)],
            })
            .enumerate()
            .map(|(i, instr)| (i % 40, instr))
            .map(|(pos, instr)| {
                let res = if pixel_sprite_overlap(pos, sprite_pos) {
                    '#'
                } else {
                    '.'
                };
                if let Instr::Addx(x) = instr {
                    sprite_pos += x;
                }
                res
            })
            .collect::<Vec<char>>();

        let crt_input = cpu_output
            .chunks(40)
            .map(|c| format!("\n{}", c.iter().collect::<String>()))
            .collect::<String>();

        format!("Day 7 part 2: {}", crt_input)
    }
}
