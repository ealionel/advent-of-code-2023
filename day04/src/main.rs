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
    let amount_winning_tickets: i32 = get_matching_numbers(&winning_numbers, &tickets);

    let base: i32 = 2;

    let power = std::cmp::max(amount_winning_tickets - 1, 0);

    if amount_winning_tickets == 0 {
        return 0;
    }

    base.pow(power as u32) as u32
}

fn get_matching_numbers(winning_numbers: &HashSet<u32>, tickets: &Vec<u32>) -> i32 {
    let matching_numbers = tickets.iter().fold(0, |sum, ticket| {
        if winning_numbers.contains(ticket) {
            sum + 1
        } else {
            sum
        }
    });

    matching_numbers
}

fn get_stratchcards(
    cards: &Vec<(HashSet<u32>, Vec<u32>)>,
    card_number: u32,
    memo: &mut Vec<i32>,
) -> i32 {
    if card_number >= cards.len() as u32 {
        return 0;
    }

    if memo[card_number as usize] != -1 {
        return memo[card_number as usize];
    }

    let (winning_numbers, tickets) = cards.get(card_number as usize).unwrap();

    let matching_numbers = get_matching_numbers(winning_numbers, tickets);

    // println!(
    //     "Card #{} matching numbers: {}, wn={:?} tickets={:?}",
    //     card_number, matching_numbers, winning_numbers, tickets
    // );

    let mut stretchcards: i32 = 1;

    for i in 1..=matching_numbers {
        stretchcards += get_stratchcards(cards, card_number + i as u32, memo);
    }

    (*memo)[card_number as usize] = stretchcards;

    stretchcards
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let answer1: u32 = input
        .lines()
        .fold(0, |sum, line| sum + get_line_value(line));

    let cards: Vec<(HashSet<u32>, Vec<u32>)> = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<(HashSet<u32>, Vec<u32>)>>();

    let mut stretchcards = 0;

    let mut memo: Vec<i32> = vec![-1; cards.len()];

    for i in 0..cards.len() {
        stretchcards += get_stratchcards(&cards, i as u32, &mut memo);
    }
    // stretchcards += get_stratchcards(&cards, 0 as u32);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", stretchcards);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_value() {
        assert_eq!(
            get_line_value("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );

        assert_eq!(
            get_line_value("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
    }

    #[test]
    fn test_parse_line() {
        let (winning_numbers, tickets) =
            parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(winning_numbers.len(), 5);
        assert_eq!(tickets.len(), 8);
    }
}
