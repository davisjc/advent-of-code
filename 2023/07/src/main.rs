use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, Debug)]
struct Hand {
    cards: [u8; 5],
    t: HandType,
    bid: u64,
}

impl Hand {
    fn new(line: &str) -> Hand {
        let mut cards: [u8; 5] = [0; 5];
        for (i, c) in line[..5].char_indices() {
            cards[i] = match c {
                'J' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("unexpected card value: {c}")
            };
        }
        let mut count_by_card: HashMap<u8, u8> = HashMap::new();
        let mut joker_count: u8 = 0;
        for card in cards {
            if card == 1 {
                joker_count += 1;
                continue;
            }
            if !count_by_card.contains_key(&card) {
                count_by_card.insert(card, 0);
            }
            *count_by_card.get_mut(&card).unwrap() += 1;
        }
        let t = match count_by_card.len() {
            0 | 1 => HandType::FiveOfAKind,
            2 => {
                if count_by_card.values().any(|count| {
                    count + joker_count == 4
                }) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            },
            3 => {
                if count_by_card.values().any(|count| {
                    count + joker_count == 3
                }) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("invalid hand structure: {cards:?}")
        };
        let bid = line[6..].parse::<u64>().unwrap();
        Hand { cards, t, bid }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t == other.t {
            self.cards.cmp(&other.cards)
        } else {
            self.t.cmp(&other.t)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut hands: Vec<Hand> = io::stdin()
        .lines()
        .map(|l| Hand::new(&l.unwrap()))
        .collect();
    hands.sort();
    let total: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u64)
        .sum();
    println!("{total}");
}
