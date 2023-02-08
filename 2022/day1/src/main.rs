use std::{fs, collections::btree_map::Iter};

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Should be able to read file");

    let nums = contents
        .split("\n\n")
        .map(|group| {
            return group
                .split("\n")
                .map(|num| num.parse::<i32>().expect("To be a number"))
                .sum::<i32>();
        });
}
