use std::{fs, char, collections::HashMap};

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/7/input").expect("Cannot read input file");
    let result = calc_ways_to_beat_the_record(&input);
    println!("Result: {}", result);
    assert!(result == 254494947);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    J,
    Numbers(u32),
    T,
    Q,
    K,
    A,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rank {
    HighCard(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card),
    TwoPair(Card, Card, Card),
    ThreeOfAKind(Card, Card, Card),
    FullHouse(Card, Card),
    FourOfAKind(Card, Card),
    FiveOfAKind(Card),
}

impl Rank {
    fn value(&self) -> u8 {
        match self {
            Self::HighCard(_, _, _, _, _) => 1,
            Self::OnePair(_, _, _, _) => 2,
            Self::TwoPair(_, _, _) => 3,
            Self::ThreeOfAKind(_, _, _) => 4,
            Self::FullHouse(_, _) => 5,
            Self::FourOfAKind(_, _) => 6,
            Self::FiveOfAKind(_) => 7,
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {Some(self.cmp(other))}
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl Card {
    fn parse(input: char) -> Result<Self, &'static str> {
        let card = match input {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            _ => match input.to_digit(10) {
                Some(1) | Some(0) => return Err("DAMN 1"),
                Some(n) => Self::Numbers(n),
                _ => return Err("DAMN"),
            },
        };
        Ok(card)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    rank: Rank,
    cards: [Card; 5],
}

impl Hand {
    fn parse(input: &str) -> Self {
        let cards: Vec<Card> = input.chars()
            .map(|c| Card::parse(c).unwrap())
            .collect();

        let mut card_counts: Vec<(Card, i32)> = cards.iter()
            .fold(HashMap::new(), |mut acc, c| {
                acc.entry(c)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                acc
            })
            .into_iter()
            .map(|(a, b)| (*a, b))
            .collect();

        card_counts.sort_by(|a, b| b.0.cmp(&a.0));
        let joker = card_counts.get(card_counts.len() - 1);
        match joker {
            Some((Card::J, 5)) => (),
            Some((Card::J, count)) => {
                let joker_count = count.to_owned();
                card_counts.remove(card_counts.len() - 1);

                card_counts.sort_by(|a, b| b.1.cmp(&a.1));
                card_counts.first_mut().unwrap().1 += joker_count;
            },
            _ => (),
        }

        let mut cc: Vec<(i32, Card)> = card_counts.iter()
            .map(|(a, b)| (*b, *a))
            .collect();

        cc.sort_by(|a, b| match a.0 == b.0 {
            true => b.1.cmp(&a.1),
            false => b.0.cmp(&a.0),
        });

        let rank: Rank = match &cc[..] {
            [(5, a)] => Rank::FiveOfAKind(*a),
            [(4, a), (1, b)] => Rank::FourOfAKind(*a, *b),
            [(3, a), (2, b)] => Rank::FullHouse(*a, *b),
            [(3, a), (1, b), (1, c)] => Rank::ThreeOfAKind(*a, *b, *c),
            [(2, a), (2, b), (1, c)] => Rank::TwoPair(*a, *b, *c),
            [(2, a), (1, b), (1, c), (1, d)] => Rank::OnePair(*a, *b, *c, *d),
            [(1, a), (1, b), (1, c), (1, d), (1, e)] => Rank::HighCard(*a, *b, *c, *d, *e),
            _ => panic!(""),
        };

        Hand {
            rank,
            cards: cards.try_into().unwrap(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Bet {
    hand: Hand,
    bid: i32,
}

impl Bet {
    fn parse(input: &str) -> Self {
        let (hand, stake) = input.split_once(" ").unwrap();
        Bet {
            hand: Hand::parse(hand),
            bid: stake.parse::<i32>().unwrap(),
        }
    }
}

fn calc_ways_to_beat_the_record(input: &str) -> i32 {
    let mut data = input.lines()
        .map(|line| Bet::parse(line))
        .collect::<Vec<Bet>>();
    data.sort_by(|a, b| a.cmp(&b));

    let mut last_bet: Option<Bet> = None;
    for (_index, &bet) in data.iter().enumerate() {
        if let None = last_bet {
            last_bet = Some(bet);
            continue;
        }
        last_bet = Some(bet);
    }

    data.iter().enumerate()
        .zip(1i32..)
        .map(|((_, bet), rank)| rank * bet.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{calc_ways_to_beat_the_record, Hand, Rank, Card};

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/7/testinput").expect("Cannot read input file");
        let ways_to_beat_record = calc_ways_to_beat_the_record(&input);
        assert!(ways_to_beat_record == 5905);
    }

    #[test]
    fn gets_converted_correctly() {
        let ordering = Hand::parse("46J24")
            .rank
            .cmp(&Rank::ThreeOfAKind(Card::Numbers(4), Card::Numbers(6), Card::Numbers(2)));
        assert_eq!(ordering, Ordering::Equal);
    }
}

