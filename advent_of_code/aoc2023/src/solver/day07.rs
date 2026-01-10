//! # Day 7: Camel Cards
//!
//! ## Part 1
//! Rank poker-style hands and calculate total winnings.
//! Each hand has a type (five of a kind, four of a kind, etc.) and a bid.
//! Hands are ranked by type first, then by card-by-card comparison.
//! Total winnings = sum of (rank * bid) for all hands.
//!
//! ## Part 2
//! J cards are now Jokers - wildcards that make the strongest hand possible.
//! J is now the weakest card for tie-breaking.
//!
//! ## Algorithm
//! - Parse hands and bids
//! - Determine hand type (using card frequency counts)
//! - Part 2: Add joker count to most frequent card for optimal hand
//! - Sort hands by (type, card values)
//! - Calculate winnings as rank * bid
//! - Complexity: O(n log n) for sorting
//!
//! ## Rust Concepts
//! - Custom ordering with Ord/PartialOrd traits
//! - HashMap for card counting (Mission 5 concept)
//! - Enum for hand types with natural ordering

use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashMap;

/// Card values for standard Camel Cards (Part 1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

/// Card values for Part 2 where Jacks are Jokers (weakest card)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card2 {
    Joker = 1, // J is now weakest
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card2 {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'J' => Some(Card2::Joker),
            '2' => Some(Card2::Two),
            '3' => Some(Card2::Three),
            '4' => Some(Card2::Four),
            '5' => Some(Card2::Five),
            '6' => Some(Card2::Six),
            '7' => Some(Card2::Seven),
            '8' => Some(Card2::Eight),
            '9' => Some(Card2::Nine),
            'T' => Some(Card2::Ten),
            'Q' => Some(Card2::Queen),
            'K' => Some(Card2::King),
            'A' => Some(Card2::Ace),
            _ => None,
        }
    }
}

/// Hand types in Camel Cards, ordered from weakest to strongest
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

/// A hand of 5 cards with a bid amount (Part 1)
#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u64) -> Self {
        let hand_type = Self::determine_type(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    /// Determine hand type by counting card frequencies
    fn determine_type(cards: &[Card; 5]) -> HandType {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for &card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut frequencies: Vec<usize> = counts.values().copied().collect();
        frequencies.sort_by(|a, b| b.cmp(a)); // Sort descending

        match frequencies.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }

        let cards_str = parts[0];
        let bid = parts[1].parse().ok()?;

        if cards_str.len() != 5 {
            return None;
        }

        let cards: Vec<Card> = cards_str.chars().filter_map(Card::from_char).collect();
        if cards.len() != 5 {
            return None;
        }

        Some(Hand::new(
            [cards[0], cards[1], cards[2], cards[3], cards[4]],
            bid,
        ))
    }
}

/// Hands are ordered first by type, then by card values left-to-right
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                // Compare cards left to right
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A hand for Part 2 with Joker rules
#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand2 {
    cards: [Card2; 5],
    bid: u64,
    hand_type: HandType,
}

impl Hand2 {
    fn new(cards: [Card2; 5], bid: u64) -> Self {
        let hand_type = Self::determine_type(&cards);
        Hand2 {
            cards,
            bid,
            hand_type,
        }
    }

    /// Determine hand type with Joker rules
    /// Jokers become whatever card makes the best hand
    fn determine_type(cards: &[Card2; 5]) -> HandType {
        let mut counts: HashMap<Card2, usize> = HashMap::new();
        let mut joker_count = 0;

        for &card in cards {
            if card == Card2::Joker {
                joker_count += 1;
            } else {
                *counts.entry(card).or_insert(0) += 1;
            }
        }

        // If all jokers, it's five of a kind
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        // Add jokers to the most frequent card to maximize hand strength
        let mut frequencies: Vec<usize> = counts.values().copied().collect();
        frequencies.sort_by(|a, b| b.cmp(a)); // Sort descending
        frequencies[0] += joker_count; // Add jokers to most frequent

        match frequencies.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }

        let cards_str = parts[0];
        let bid = parts[1].parse().ok()?;

        if cards_str.len() != 5 {
            return None;
        }

        let cards: Vec<Card2> = cards_str.chars().filter_map(Card2::from_char).collect();
        if cards.len() != 5 {
            return None;
        }

        Some(Hand2::new(
            [cards[0], cards[1], cards[2], cards[3], cards[4]],
            bid,
        ))
    }
}

/// Hands for Part 2 are ordered first by type, then by card values (J is weakest)
impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                // Compare cards left to right
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// **Day 7: Camel Cards - Part 1**
pub fn solve_part1(input: &str) -> Result<String> {
    let mut hands: Vec<Hand> = input.lines().filter_map(Hand::parse).collect();

    // Sort hands from weakest to strongest
    hands.sort();

    // Calculate total winnings: rank * bid
    let total_winnings: u64 = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let rank = (index + 1) as u64;
            rank * hand.bid
        })
        .sum();

    Ok(total_winnings.to_string())
}

/// **Day 7: Camel Cards - Part 2 with Joker rules**
pub fn solve_part2(input: &str) -> Result<String> {
    let mut hands: Vec<Hand2> = input.lines().filter_map(Hand2::parse).collect();

    // Sort hands from weakest to strongest
    hands.sort();

    // Calculate total winnings: rank * bid
    let total_winnings: u64 = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let rank = (index + 1) as u64;
            rank * hand.bid
        })
        .sum();

    Ok(total_winnings.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "6440");
    }

    #[test]
    fn test_hand_type_detection() {
        // Five of a kind
        let hand = Hand::parse("AAAAA 1").unwrap();
        assert_eq!(hand.hand_type, HandType::FiveOfAKind);

        // Four of a kind
        let hand = Hand::parse("AA8AA 1").unwrap();
        assert_eq!(hand.hand_type, HandType::FourOfAKind);

        // Full house
        let hand = Hand::parse("23332 1").unwrap();
        assert_eq!(hand.hand_type, HandType::FullHouse);

        // Three of a kind
        let hand = Hand::parse("TTT98 1").unwrap();
        assert_eq!(hand.hand_type, HandType::ThreeOfAKind);

        // Two pair
        let hand = Hand::parse("23432 1").unwrap();
        assert_eq!(hand.hand_type, HandType::TwoPair);

        // One pair
        let hand = Hand::parse("A23A4 1").unwrap();
        assert_eq!(hand.hand_type, HandType::OnePair);

        // High card
        let hand = Hand::parse("23456 1").unwrap();
        assert_eq!(hand.hand_type, HandType::HighCard);
    }

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand::parse("33332 1").unwrap();
        let hand2 = Hand::parse("2AAAA 1").unwrap();
        assert!(hand1 > hand2); // 33332 stronger (first card higher)

        let hand1 = Hand::parse("77888 1").unwrap();
        let hand2 = Hand::parse("77788 1").unwrap();
        assert!(hand1 > hand2); // 77888 stronger (third card higher)
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "5905");
    }

    #[test]
    fn test_joker_hand_type() {
        // QJJQ2 -> Four of a kind (Js become Q)
        let hand = Hand2::parse("QJJQ2 1").unwrap();
        assert_eq!(hand.hand_type, HandType::FourOfAKind);

        // T55J5 -> Four of a kind (J becomes 5)
        let hand = Hand2::parse("T55J5 1").unwrap();
        assert_eq!(hand.hand_type, HandType::FourOfAKind);

        // KTJJT -> Four of a kind (Js become T)
        let hand = Hand2::parse("KTJJT 1").unwrap();
        assert_eq!(hand.hand_type, HandType::FourOfAKind);

        // QQQJA -> Four of a kind (J becomes Q)
        let hand = Hand2::parse("QQQJA 1").unwrap();
        assert_eq!(hand.hand_type, HandType::FourOfAKind);

        // 32T3K -> One pair (no jokers)
        let hand = Hand2::parse("32T3K 1").unwrap();
        assert_eq!(hand.hand_type, HandType::OnePair);

        // JJJJJ -> Five of a kind (all jokers)
        let hand = Hand2::parse("JJJJJ 1").unwrap();
        assert_eq!(hand.hand_type, HandType::FiveOfAKind);
    }

    #[test]
    fn test_joker_ordering() {
        // JKKK2 vs QQQQ2 - both four of a kind, but Q > J in first position
        let hand1 = Hand2::parse("JKKK2 1").unwrap();
        let hand2 = Hand2::parse("QQQQ2 1").unwrap();
        assert!(hand1 < hand2); // QQQQ2 stronger (Q > J)
    }
}
