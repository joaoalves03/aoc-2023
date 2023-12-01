advent_of_code::solution!(1);
use std::str::{Split};
use fancy_regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let data: Split<&str> = input.split("\n");

    let mut total = 0;

    for line in data {
        for c in line.chars() {
            if c.is_digit(10) {
                total += c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                total += c.to_digit(10).unwrap();
                break;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let re = Regex::new(r"(?=([1-9]|one|two|three|four|five|six|seven|eight|nine))").unwrap();

    for line in input.lines() {
        let caps: Vec<u32> = re.captures_iter(line).map(|x| {
            let mut val = x.unwrap().get(1).unwrap().as_str();

            match val {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => val.parse::<u32>().unwrap()
            }
        }).collect();

        total += caps.first().unwrap() * 10;
        total += caps.last().unwrap();
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
