use crate::utils::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn upgrade_handtype(hand_type: HandType) -> HandType {
    use HandType::*;
    match hand_type {
        HighCard => OnePair,
        OnePair => ThreeOfKind,
        TwoPair => FullHouse,
        ThreeOfKind => FourOfKind,
        FullHouse => FourOfKind,
        FourOfKind => FiveOfKind,
        FiveOfKind => FiveOfKind,
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    bid: u32,
}

impl Hand {
    fn from_inputs(cards: &str, bid: u32, with_joker: bool) -> Self {
        let cards_array = Self::get_cards_array(cards, with_joker);
        let calculated_hand_type = Self::calc_hand_type(cards_array);

        Self {
            cards: cards_array,
            hand_type: calculated_hand_type,
            bid,
        }
    }

    fn calc_hand_type(cards: [u8; 5]) -> HandType {
        let seen = cards.iter().fold(HashMap::new(), |mut acc, &i| {
            *acc.entry(i).or_insert(0) += 1;
            acc
        });
        let n_jokers = cards.iter().filter(|&&c| c == 1).count();
        let res = match seen.len() {
            1 => HandType::FiveOfKind,
            2 => {
                if seen.values().any(|&v| v == 4) {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if seen.values().any(|&v| v == 3) {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };

        match (n_jokers, &res) {
            (1, _) => upgrade_handtype(res),
            (2, HandType::OnePair) => HandType::ThreeOfKind,
            (2, HandType::TwoPair) => HandType::FourOfKind,
            (2, HandType::ThreeOfKind) => HandType::FiveOfKind,
            (2, HandType::FullHouse) => HandType::FiveOfKind,
            (3, HandType::ThreeOfKind) => HandType::FourOfKind,
            (3, HandType::FullHouse) => HandType::FiveOfKind,
            (4, _) => HandType::FiveOfKind,
            _ => res,
        }
    }

    fn get_cards_array(cards: &str, with_joker: bool) -> [u8; 5] {
        assert!(cards.len() == 5);
        let card_repr = if with_joker {
            "J23456789TQKA"
        } else {
            "23456789TJQKA"
        };
        let mut res = [0; 5];
        for (i, c) in cards.chars().enumerate() {
            res[i] = card_repr.find(c).unwrap() as u8;
            if with_joker {
                res[i] += 1u8
            } else {
                res[i] += 2u8
            }
        }
        res
    }
}

pub fn solve_d7() -> (u32, u32) {
    let lines = read_lines("data/d07.txt");
    let mut hands_p1 = Vec::with_capacity(lines.len());
    let mut hands_p2 = Vec::with_capacity(lines.len());
    for line in lines {
        let mut sp = line.split(' ');
        let sp1 = sp.next().unwrap();
        let sp2 = sp.next().unwrap();
        hands_p1.push(Hand::from_inputs(sp1, str_to::<u32>(sp2), false));
        hands_p2.push(Hand::from_inputs(sp1, str_to::<u32>(sp2), true));
    }
    hands_p1.sort();
    hands_p2.sort();
    let mut res_p1 = 0;
    let mut res_p2 = 0;
    for idx in 0..hands_p1.len() {
        res_p1 += (idx + 1) * hands_p1[idx].bid as usize;
        res_p2 += (idx + 1) * hands_p2[idx].bid as usize;
    }
    //250898830, 252127335
    (res_p1 as u32, res_p2 as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_type_sort_test() {
        assert!(HandType::FiveOfKind > HandType::FourOfKind);
        assert!(HandType::TwoPair < HandType::FourOfKind);
        assert!(HandType::FullHouse == HandType::FullHouse);
    }
    #[test]
    fn get_cards_array_test() {
        // no joker
        assert_eq!(Hand::get_cards_array("23456", false), [2, 3, 4, 5, 6]);
        assert_eq!(Hand::get_cards_array("TJQKA", false), [10, 11, 12, 13, 14]);

        // with joker
        assert_eq!(Hand::get_cards_array("TJQKA", true), [10, 1, 11, 12, 13]);
    }
    #[test]
    fn comp_hands_test() {
        assert!(Hand::from_inputs(&"JKKK2", 1, true) < Hand::from_inputs(&"QQQQ2", 1, true));
        assert!(Hand::from_inputs(&"T55J5", 1, true) < Hand::from_inputs(&"QQQJA", 1, true));
        assert!(Hand::from_inputs(&"QQQJA", 1, true) < Hand::from_inputs(&"KTJJT", 1, true));

        assert!(Hand::from_inputs(&"KKK23", 1, true) < Hand::from_inputs(&"AJJ94", 1, true));
        assert!(Hand::from_inputs(&"AJJ94", 1, true) < Hand::from_inputs(&"A2223", 1, true));

        assert!(Hand::from_inputs(&"JJJJJ", 1, true) < Hand::from_inputs(&"2JJJJ", 1, true));
    }
    #[test]
    fn calc_hand_type_test() {
        // no joker
        assert_eq!(Hand::calc_hand_type([6, 6, 6, 6, 6]), HandType::FiveOfKind);
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("AAAAA", false)),
            HandType::FiveOfKind
        );

        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("AAQQQ", false)),
            HandType::FullHouse
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("23332", false)),
            HandType::FullHouse
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("43434", false)),
            HandType::FullHouse
        );

        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("43442", false)),
            HandType::ThreeOfKind
        );

        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("43432", false)),
            HandType::TwoPair
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("AQAQ2", false)),
            HandType::TwoPair
        );

        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("44532", false)),
            HandType::OnePair
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("4A53A", false)),
            HandType::OnePair
        );

        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("A2345", false)),
            HandType::HighCard
        );

        // with joker
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("AAJAA", true)),
            HandType::FiveOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("AJJAA", true)),
            HandType::FiveOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("A3JAA", true)),
            HandType::FourOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("JAA45", true)),
            HandType::ThreeOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("2345J", true)),
            HandType::OnePair
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("2342J", true)),
            HandType::ThreeOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("23J2J", true)),
            HandType::FourOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("23J2J", true)),
            HandType::FourOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("JJJJJ", true)),
            HandType::FiveOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("J4JJJ", true)),
            HandType::FiveOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("QQQQ2", true)),
            HandType::FourOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("JKKK2", true)),
            HandType::FourOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("J2K77", true)),
            HandType::ThreeOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("JJ423", true)),
            HandType::ThreeOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("QQKKJ", true)),
            HandType::FullHouse
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("AJJ94", true)),
            HandType::ThreeOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("2233J", true)),
            HandType::FullHouse
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("4T48J", true)),
            HandType::ThreeOfKind
        );
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("J2345", true)),
            HandType::OnePair
        );
        // FUCK!
        assert_eq!(
            Hand::calc_hand_type(Hand::get_cards_array("JJJ34", true)),
            HandType::FourOfKind
        );
    }
}
