use std::fs::read_to_string;

fn extract_numbers(s: &str) -> u32 {
    let buf: Vec<char> = s
        .chars()
        .filter(|ch| ch.to_digit(10).is_some())
        .collect();

    let first = *buf.first().unwrap_or(&'0');
    let second = *buf.last().unwrap_or(&first);

    format!("{}{}", first, second)
        .parse::<u32>()
        .unwrap()
}

fn main() {
    let source = read_to_string("input.txt").unwrap();
    let num: u32 = source
        .as_str()
        .lines()
        .map(extract_numbers)
        .sum();

    println!("{:?}", num);
}
