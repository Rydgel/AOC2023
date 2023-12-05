const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<String> {
    input.split('\n').map(|s| s.to_string()).collect()
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| {
            let a = line.chars().find(|&c| c.is_ascii_digit()).unwrap_or('0');
            let b = line.chars().rfind(|&c| c.is_ascii_digit()).unwrap_or('0');
            format!("{}{}", a, b).parse().unwrap_or(0)
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
    let input: Vec<String> = input.iter().map(|line| clean_string(line)).collect();
    part1(&input)
}

fn clean_string(line: &str) -> String {
    let mut line = line.to_owned();

    let replaces = [
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];

    for replace_data in replaces {
        line = line.replace(replace_data.0, replace_data.1);
    }

    line
}

fn main() {
    let input = parse_input(INPUT);
    println!("day1 p1: {:?}", part1(&input));
    println!("day1 p2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        const TEST_INPUT: &str = include_str!("../test_input.txt");
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 142);
    }

    #[test]
    fn p2() {
        const TEST_INPUT: &str = include_str!("../test_input2.txt");
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 281);
    }
}
