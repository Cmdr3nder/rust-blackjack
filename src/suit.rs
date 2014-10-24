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
            Clubs => write!(f, "Clubs"),
            Diamonds => write!(f, "Diamonds"),
            Spades => write!(f, "Spades"),
            Hearts => write!(f, "Hearts")
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