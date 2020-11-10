use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    Deuce,
    Trey,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Deuce => "2",
                Rank::Trey => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "T",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            }
        )
    }
}

impl FromStr for Rank {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Rank::Deuce),
            "3" => Ok(Rank::Trey),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" | "t" => Ok(Rank::Ten),
            "J" | "j" => Ok(Rank::Jack),
            "Q" | "q" => Ok(Rank::Queen),
            "K" | "k" => Ok(Rank::King),
            "A" | "a" => Ok(Rank::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Club => "c",
                Suit::Diamond => "d",
                Suit::Heart => "h",
                Suit::Spade => "s",
            }
        )
    }
}

impl FromStr for Suit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "c" | "C" => Ok(Suit::Club),
            "d" | "D" => Ok(Suit::Diamond),
            "h" | "H" => Ok(Suit::Heart),
            "s" | "S" => Ok(Suit::Spade),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn id(&self) -> usize {
        let rank_id = match self.rank {
            Rank::Deuce => 0,
            Rank::Trey => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        };
        let suit_id = match self.suit {
            Suit::Club => 0,
            Suit::Diamond => 1,
            Suit::Heart => 2,
            Suit::Spade => 3,
        };
        4 * rank_id + suit_id
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let rank = chars.next().ok_or(())?.to_string().parse::<Rank>()?;
        let suit = chars.next().ok_or(())?.to_string().parse::<Suit>()?;
        if !chars.next().is_none() {
            return Err(());
        }
        Ok(Card { rank, suit })
    }
}

pub fn parse_hand(s: &str) -> Result<Vec<Card>, ()> {
    let len = s.len();
    let mut cards = Vec::new();
    if len % 2 == 1 {
        return Err(());
    }
    for i in (0..len).step_by(2) {
        let card = s.get(i..(i + 2)).ok_or(())?;
        cards.push(card.parse::<Card>()?)
    }
    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            "5d".parse::<Card>(),
            Ok(Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            })
        );
        assert_eq!(
            "Qh".parse::<Card>(),
            Ok(Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            })
        );
        assert_eq!("Jx".parse::<Card>(), Err(()));
        assert_eq!("1s".parse::<Card>(), Err(()));
        assert_eq!("".parse::<Card>(), Err(()));
        assert_eq!("4cG".parse::<Card>(), Err(()));

        assert_eq!(parse_hand(""), Ok(Vec::new()));
        assert_eq!(
            parse_hand("2d3cKs"),
            Ok(vec![
                Card {
                    rank: Rank::Deuce,
                    suit: Suit::Diamond,
                },
                Card {
                    rank: Rank::Trey,
                    suit: Suit::Club,
                },
                Card {
                    rank: Rank::King,
                    suit: Suit::Spade,
                },
            ])
        );
        assert_eq!(parse_hand("2d3cKsA"), Err(()));
    }
}
