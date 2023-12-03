use std::collections::HashMap;
use std::format;
use std::fs;

fn get_line_value(line: &str) -> i32 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    let first_digit = digits.first().unwrap_or(&'0');
    let last_digit = digits.last().unwrap_or(&'0');

    format!("{}{}", first_digit, last_digit)
        .parse::<i32>()
        .expect("Could not parse line value")
}

fn get_line_value_2(line: &str) -> i32 {
    let mapping = HashMap::from([
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let to_match: Vec<&str> = mapping.keys().cloned().collect();

    let mut min_index = usize::MAX;
    let mut max_index = usize::MIN;

    let mut first_digit = 'n';
    let mut last_digit = 'n';

    for word in to_match.iter() {
        match line.find(word) {
            Some(index) => {
                if index <= min_index {
                    min_index = index;
                    first_digit = *mapping.get(word).expect("Could not get mapping");
                }
            }
            None => {}
        }

        match line.rfind(word) {
            Some(index) => {
                if index >= max_index {
                    max_index = index;
                    last_digit = *mapping.get(word).expect("Could not get mapping");
                }
            }
            None => {}
        }
    }

    format!("{}{}", first_digit, last_digit)
        .parse::<i32>()
        .expect("Could not parse line value")
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let answer1 = input
        .lines()
        .fold(0, |acc, line| get_line_value(line) + acc);

    let answer2 = input
        .lines()
        .fold(0, |acc, line| get_line_value_2(line) + acc);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_value() {
        assert_eq!(get_line_value("1abc2"), 12);
        assert_eq!(get_line_value("pqr3stu8vwx"), 38);
        assert_eq!(get_line_value("a1b2c3d4e5f"), 15);
        assert_eq!(get_line_value("treb7uchet"), 77);
        assert_eq!(get_line_value("blabla12blabla345"), 15);
        assert_eq!(get_line_value("54321"), 51);
        assert_eq!(get_line_value("abc123"), 13);
        assert_eq!(get_line_value("123abc"), 13);
    }

    #[test]
    fn test_get_line_value_2() {
        assert_eq!(get_line_value_2("two1nine"), 29);
        assert_eq!(get_line_value_2("eightwothree"), 83);
        assert_eq!(get_line_value_2("abcone2threexyz"), 13);
        assert_eq!(get_line_value_2("xtwone3four"), 24);
        assert_eq!(get_line_value_2("4nineeightseven2"), 42);
        assert_eq!(get_line_value_2("zoneight234"), 14);
        assert_eq!(get_line_value_2("7pqrstsixteen"), 76);
        assert_eq!(get_line_value_2("rrd8rrfcvzhcdr651rfc6h"), 88);
    }
}
