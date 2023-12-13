use std::cmp::Ordering;
use std::{collections::HashMap, fs};

#[derive(PartialEq, Debug, PartialOrd, Eq, Clone)]
enum HandType {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

impl HandType {
    fn get_value(&self) -> u8 {
        match self {
            HandType::FiveOfAKind(_) => 7,
            HandType::FourOfAKind(_) => 6,
            HandType::FullHouse(_) => 5,
            HandType::ThreeOfAKind(_) => 4,
            HandType::TwoPair(_) => 3,
            HandType::OnePair(_) => 2,
            HandType::HighCard(_) => 1,
        }
    }
}

fn extract_hand_from_enum(hand_type: &HandType) -> &str {
    match hand_type {
        HandType::FiveOfAKind(hand) => hand,
        HandType::FourOfAKind(hand) => hand,
        HandType::FullHouse(hand) => hand,
        HandType::ThreeOfAKind(hand) => hand,
        HandType::TwoPair(hand) => hand,
        HandType::OnePair(hand) => hand,
        HandType::HighCard(hand) => hand,
    }
}

fn compare_card(card1: char, card2: char) -> Ordering {
    let card1_value = match card1 {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card1.to_digit(10).unwrap(),
    };

    let card2_value: u32 = match card2 {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card2.to_digit(10).unwrap(),
    };

    card1_value.cmp(&card2_value)
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = self.get_value();
        let other_value = other.get_value();

        if self_value != other_value {
            return self.get_value().cmp(&other.get_value());
        }

        let self_hand = extract_hand_from_enum(self);
        let other_hand = extract_hand_from_enum(other);

        for (a, b) in self_hand.chars().zip(other_hand.chars()) {
            let card_cmp = compare_card(a, b);

            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }

        Ordering::Equal
    }
}

fn parse_line(line: &str) -> (HandType, u32) {
    let parts = line.split(" ").collect::<Vec<&str>>();

    let bid = parts[1].parse::<u32>().unwrap();
    let hand = get_hand_type(parts[0]);

    (hand, bid)
}

fn get_hand_type(hand: &str) -> HandType {
    let mut cards_count: HashMap<char, u32> = HashMap::new();

    for card in hand.chars() {
        let count = cards_count.entry(card).or_insert(0);
        *count += 1;
    }

    let mut counts = cards_count.values().collect::<Vec<&u32>>();

    counts.sort_by(|a, b| b.cmp(a));

    if *counts[0] == 5 {
        return HandType::FiveOfAKind(String::from(hand));
    }

    if *counts[0] == 4 {
        return HandType::FourOfAKind(String::from(hand));
    }

    if counts.len() == 2 && *counts[0] == 3 {
        return HandType::FullHouse(String::from(hand));
    }

    if counts.len() == 3 && *counts[0] == 3 {
        return HandType::ThreeOfAKind(String::from(hand));
    }

    if counts.len() == 3 && *counts[0] == 2 && *counts[1] == 2 {
        return HandType::TwoPair(String::from(hand));
    }

    if counts.len() == 4 && *counts[0] == 2 {
        return HandType::OnePair(String::from(hand));
    }

    HandType::HighCard(String::from(hand))
}

fn rank_hands(input: &mut Vec<(HandType, u32)>) -> u32 {
    let mut hands = input.clone();
    hands.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum = 0;

    for (rank, hand) in hands.iter().enumerate() {
        sum += hand.1 * (rank as u32 + 1);
    }

    sum
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut parsed: Vec<(HandType, u32)> = input.lines().map(|line| parse_line(line)).collect();

    let answer1 = rank_hands(&mut parsed);

    println!("Answer 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "32T3K 765";
        let (hand, bid) = parse_line(line);

        assert_eq!(hand, HandType::OnePair(String::from("32T3K")));
        assert_eq!(bid, 765);
    }

    #[test]
    fn test_get_hand_type() {
        assert_eq!(
            get_hand_type("32T3K"),
            HandType::OnePair(String::from("32T3K"))
        );
        assert_eq!(
            get_hand_type("T55J5"),
            HandType::ThreeOfAKind(String::from("T55J5"))
        );
        assert_eq!(
            get_hand_type("KK677"),
            HandType::TwoPair(String::from("KK677"))
        );
        assert_eq!(
            get_hand_type("KTJJT"),
            HandType::TwoPair(String::from("KTJJT"))
        );
        assert_eq!(
            get_hand_type("QQQJA"),
            HandType::ThreeOfAKind(String::from("QQQJA"))
        );
        assert_eq!(
            get_hand_type("AAAAA"),
            HandType::FiveOfAKind(String::from("AAAAA"))
        );
        assert_eq!(
            get_hand_type("AAAAB"),
            HandType::FourOfAKind(String::from("AAAAB"))
        );
        assert_eq!(
            get_hand_type("AAABB"),
            HandType::FullHouse(String::from("AAABB"))
        );
        assert_eq!(
            get_hand_type("ABCDE"),
            HandType::HighCard(String::from("ABCDE"))
        );
    }

    #[test]
    fn test_handtype_cmp() {
        let five_of_a_kind: HandType = HandType::FiveOfAKind(String::from("AAAAA"));
        let four_of_a_kind = HandType::FourOfAKind(String::from("AAAAT"));

        assert_eq!(five_of_a_kind.cmp(&four_of_a_kind), Ordering::Greater);
        assert_eq!(four_of_a_kind.cmp(&five_of_a_kind), Ordering::Less);
        assert_eq!(four_of_a_kind.cmp(&four_of_a_kind), Ordering::Equal);

        let full_house_1: HandType = HandType::FullHouse(String::from("TTAAA"));
        let full_house_2: HandType = HandType::FullHouse(String::from("AAATT"));

        assert_eq!(full_house_1.cmp(&full_house_2), Ordering::Less);
    }
}
