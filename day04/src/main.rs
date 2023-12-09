use std::{collections::HashSet, fs};

fn parse_line(line: &str) -> (HashSet<u32>, Vec<u32>) {
    let parts = &line[10..line.len()].split(" | ").collect::<Vec<&str>>();

    let (winning_numbers, tickets) = (
        parts[0]
            .split(" ")
            .filter(|n: &&str| n != &"")
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>(),
        parts[1]
            .split(" ")
            .filter(|n: &&str| n != &"")
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>(),
    );

    (winning_numbers, tickets)
}

fn get_line_value(line: &str) -> u32 {
    let (winning_numbers, tickets) = parse_line(line);

    let amount_winning_tickets: i32 = tickets.iter().fold(0, |sum, ticket| {
        if winning_numbers.contains(ticket) {
            sum + 1
        } else {
            sum
        }
    });

    println!("Amount winning tickets: {}", amount_winning_tickets);

    let base: i32 = 2;

    let power = std::cmp::max(amount_winning_tickets - 1, 0);

    if amount_winning_tickets == 0 {
        return 0;
    }

    base.pow(power as u32) as u32
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let answer1 = input
        .lines()
        .fold(0, |sum, line| sum + get_line_value(line));

    println!("Answer 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            get_line_value("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );

        assert_eq!(
            get_line_value("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
    }
}
