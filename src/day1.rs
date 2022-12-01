use std::fs;

fn get_input() -> String {
    fs::read_to_string("input/day1.txt").expect("failed to read input to string")
}

pub fn a() -> u32 {
    get_input()
        .split("\r\n\r\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<u32>().expect("failed to parse u32"))
                .sum::<u32>()
        })
        .max()
        .expect("failed to get max")
}

pub fn b() -> u32 {
    let mut calories = get_input()
        .split("\r\n\r\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<u32>().expect("failed to parse u32"))
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    calories.sort();
    calories.into_iter().rev().take(3).sum::<u32>()
}
