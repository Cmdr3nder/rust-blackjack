use std::fmt;
use rank::Rank;
use suit::Suit;

pub enum Card {
    Regular(Suit, Rank),
    Joker,
    Blank
}

impl fmt::Show for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Regular(suit, rank) => write!(f, "{}{}", rank, suit),
            Joker => write!(f, "Joker"),
            Blank => write!(f, "Blank Card")
        }
    }
}

impl Clone for Card {
    fn clone(&self) -> Card {
        match *self {
            Regular(suit, rank) => Regular(suit, rank),
            Joker => Joker,
            Blank => Blank
        }
    }
}