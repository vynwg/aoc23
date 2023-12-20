const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn parse(line: &str) -> u32 {
    let indices: Vec<_> = NUMBERS
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, num| {
            acc.extend(line
                .match_indices(num.1)
                .map(|(i, _)| (i, num.0+1))
                .collect::<Vec<_>>()
            );
            acc
        });

    let parsed: Vec<_> = line
        .chars()
        .enumerate()
        .filter_map(|(i, k)| {
            if k.is_digit(10) {
                return Some(k.to_string());
            }
            
            let index = indices.iter().find(|x| x.0 == i);

            if let Some((_, num)) = index {
                return Some(num.to_string());
            }

            return None;
        })
        .collect();

    let first = parsed.first().unwrap();
    let second = parsed.last().unwrap_or(&first);

    format!("{}{}", first, second)
        .parse::<u32>()
        .unwrap()
}

fn main() {
    let source = std::fs::read_to_string("input.txt").unwrap();
    let num: u32 = source
        .lines()
        .map(parse)
        .sum();

    println!("{:?}", num);
}
