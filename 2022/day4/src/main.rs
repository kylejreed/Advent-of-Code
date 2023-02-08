use std::{fs, str::FromStr};

mod group;

use group::Group;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &String) {
    let count = input
        .lines()
        .map(|line| Group::from_str(line).unwrap())
        .filter(|group| group.needs_reconsidering())
        .count();

    println!("Part 1: {count}")
}

fn part_2(input: &String) {
    let count = input
        .lines()
        .map(|line| Group::from_str(line).unwrap())
        .filter(|group| group.contains_overlaps())
        .count();

    println!("Part 2: {count}")
}
