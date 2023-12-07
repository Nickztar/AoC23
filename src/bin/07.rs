use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Card,
    Pair,
    TwoPair,
    ThreeKind,
    FourKind,
    FullHouse,
    FiveKind,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (other, self) {
            (Value::Card, Value::Card) => Some(Ordering::Equal),
            (Value::Card, _) => Some(Ordering::Greater),
            (Value::Pair, Value::Card) => Some(Ordering::Less),
            (Value::Pair, Value::Pair) => Some(Ordering::Equal),
            (Value::Pair, _) => Some(Ordering::Greater),
            (Value::ThreeKind, Value::Card) => Some(Ordering::Less),
            (Value::ThreeKind, Value::Pair) => Some(Ordering::Less),
            (Value::ThreeKind, Value::TwoPair) => Some(Ordering::Less),
            (Value::ThreeKind, Value::ThreeKind) => Some(Ordering::Equal),
            (Value::ThreeKind, _) => Some(Ordering::Greater),
            (Value::FourKind, Value::FourKind) => Some(Ordering::Equal),
            (Value::FourKind, Value::FiveKind) => Some(Ordering::Greater),
            (Value::FourKind, _) => Some(Ordering::Less),
            (Value::FullHouse, Value::FullHouse) => Some(Ordering::Equal),
            (Value::FullHouse, Value::FiveKind) => Some(Ordering::Greater),
            (Value::FullHouse, Value::FourKind) => Some(Ordering::Greater),
            (Value::FullHouse, _) => Some(Ordering::Less),
            (Value::FiveKind, Value::FiveKind) => Some(Ordering::Equal),
            (Value::FiveKind, _) => Some(Ordering::Less),
            (Value::TwoPair, Value::Card) => Some(Ordering::Less),
            (Value::TwoPair, Value::Pair) => Some(Ordering::Less),
            (Value::TwoPair, Value::TwoPair) => Some(Ordering::Equal),
            (Value::TwoPair, _) => Some(Ordering::Greater),
        }
    }
}

fn hand_value(hand: &str) -> Value {
    let group: HashMap<char, u32> = hand.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let mut v = Value::Card;
    for (_, count) in group {
        let expected = match count {
            2 if v == Value::ThreeKind => Value::FullHouse,
            2 if v == Value::Pair => Value::TwoPair,
            2 => Value::Pair,
            3 if v == Value::Pair => Value::FullHouse,
            3 => Value::ThreeKind,
            4 => Value::FourKind,
            5 => Value::FiveKind,
            _ => Value::Card,
        };

        if expected > v {
            v = expected;
        }
    }

    v
}

fn hand_value_2(hand: &str) -> Value {
    let mut group: HashMap<char, u32> = hand.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let jokers = group.remove(&'J').unwrap_or(0);
    let mut sorted = group.iter().map(|s| s.1).collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.cmp(a));
    let top = **sorted.first().unwrap_or(&&0) + jokers;
    match top {
        5 => Value::FiveKind,
        4 => Value::FourKind,
        3 => {
            if sorted[1] == &2 {
                return Value::FullHouse;
            } else {
                return Value::ThreeKind;
            }
        }
        2 => {
            if sorted[1] == &2 {
                return Value::TwoPair;
            } else {
                return Value::Pair;
            }
        }
        1 => Value::Card,
        _ => unreachable!(),
    }
}

fn compare_eq(map: &HashMap<char, usize>, hand1: &str, hand2: &str) -> Ordering {
    for (a, b) in hand1
        .chars()
        .zip(hand2.chars())
        .map(|(a, b)| (map.get(&a).unwrap(), map.get(&b).unwrap()))
    {
        return match b.cmp(a) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => continue,
            Ordering::Greater => Ordering::Greater,
        };
    }
    Ordering::Equal
}

pub fn part_one(input: &str) -> Option<u32> {
    let order = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .into_iter()
    .enumerate()
    .rev()
    .map(|(x, y)| (y, x))
    .collect::<HashMap<char, usize>>();

    let mut a = input
        .lines()
        .map(|s| {
            let (card, score) = s.split_once(' ').unwrap();
            (hand_value(card), card, score.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    a.sort_by(|a, b| match a.0.partial_cmp(&b.0).unwrap() {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => compare_eq(&order, a.1, b.1),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });
    dbg!(&a);
    Some(
        a.into_iter()
            .enumerate()
            .map(|(i, (_, _, l))| l * (i + 1))
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let order = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ]
    .into_iter()
    .enumerate()
    .rev()
    .map(|(x, y)| (y, x))
    .collect::<HashMap<char, usize>>();

    let mut a = input
        .lines()
        .map(|s| {
            let (card, score) = s.split_once(' ').unwrap();
            (hand_value_2(card), card, score.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    a.sort_by(|a, b| match a.0.partial_cmp(&b.0).unwrap() {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => compare_eq(&order, a.1, b.1),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });
    dbg!(&a);
    Some(
        a.into_iter()
            .enumerate()
            .map(|(i, (_, _, l))| l * (i + 1))
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
        assert_eq!(hand_value_2("JJJJJ"), Value::FiveKind);
        assert_eq!(hand_value_2("2JJJJ"), Value::FiveKind);
        assert_eq!(hand_value_2("JJ234"), Value::ThreeKind);
        assert_eq!(hand_value_2("J5234"), Value::Pair);
        assert_eq!(hand_value_2("JJ334"), Value::FourKind);
        assert_eq!(hand_value_2("22JJK"), Value::FourKind);
        assert_eq!(hand_value_2("22JJK"), Value::FourKind);
        assert_eq!(hand_value_2("22JKK"), Value::FullHouse);
    }
}

/*
Homemade example
32TQ2 765
32TQ3 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
KKKKK 10
QQQQT 11
QQQQK 11
TTTKK 10
KKTTT 20
TTTTT 11
*/
