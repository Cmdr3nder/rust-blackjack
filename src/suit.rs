use std::fmt;

pub enum Suit {
    Clubs,
    Diamonds,
    Spades,
    Hearts
}

impl fmt::Show for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Clubs => write!(f, "♣"),
            Diamonds => write!(f, "♦"),
            Spades => write!(f, "♠"),
            Hearts => write!(f, "♥")
        }
    }
}

pub fn suits_to_vector() -> Vec<Suit> {
    vec![
        Clubs,
        Diamonds,
        Spades,
        Hearts
    ]
}