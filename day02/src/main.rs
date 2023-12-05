use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Game {
    game_number: usize,
    plays: Vec<Play>,
}

#[derive(Debug)]
struct Play {
    red: usize,
    green: usize,
    blue: usize,
}

impl From<&str> for Game {
    fn from(item: &str) -> Self {
        let re = Regex::new(r"Game (\d+): (.*)").unwrap();
        let captures = re.captures(item).unwrap();

        Game {
            game_number: captures[1].parse().unwrap(),
            plays: captures[2].split(';').map(|p| p.into()).collect(),
        }
    }
}

impl From<&str> for Play {
    fn from(item: &str) -> Self {
        let re_red = Regex::new(r"(\d+) red").unwrap().captures(item);
        let re_green = Regex::new(r"(\d+) green").unwrap().captures(item);
        let re_blue = Regex::new(r"(\d+) blue").unwrap().captures(item);

        let red = re_red.map(|c| c[1].parse().unwrap()).unwrap_or(0);
        let green = re_green.map(|c| c[1].parse().unwrap()).unwrap_or(0);
        let blue = re_blue.map(|c| c[1].parse().unwrap()).unwrap_or(0);

        Play { red, green, blue }
    }
}

const POSSIBLES: (usize, usize, usize) = (12, 13, 14);

fn parse_input(input: &str) -> Vec<Game> {
    input.split('\n').map(|s| s.into()).collect()
}

fn part1(input: &[Game]) -> usize {
    input
        .iter()
        .filter(|g| is_play_possible(&g.plays))
        .map(|g| g.game_number)
        .sum()
}

fn is_play_possible(plays: &[Play]) -> bool {
    for play in plays {
        if play.red > POSSIBLES.0 || play.green > POSSIBLES.1 || play.blue > POSSIBLES.2 {
            return false;
        }
    }

    true
}

fn part2(input: &[Game]) -> usize {
    0
}

fn main() {
    let input = parse_input(INPUT);
    println!("day02 p1: {:?}", part1(&input));
    println!("day02 p2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 2_286);
    }
}
