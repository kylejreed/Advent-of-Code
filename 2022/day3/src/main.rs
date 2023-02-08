fn main() {
    println!("Part 1: {}", part_1());
}

fn part_1() -> u32 {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    return input
        .lines()
        .filter_map(|line| {
            let line = line.as_bytes();
            let (l, r) = line.split_at(line.len() / 2);
            l.iter()
                .find(|item| r.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
        .sum();
}
