use std::fmt;

pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl fmt::Show for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ace => write!(f, "A"),
            Two => write!(f, "2"),
            Three => write!(f, "3"),
            Four => write!(f, "4"),
            Five => write!(f, "5"),
            Six => write!(f, "6"),
            Seven => write!(f, "7"),
            Eight => write!(f, "8"),
            Nine => write!(f, "9"),
            Ten => write!(f, "10"),
            Jack => write!(f, "J"),
            Queen => write!(f, "Q"),
            King => write!(f, "K")
        }
    }
}

pub fn ranks_to_vector() -> Vec<Rank> {
    vec![
        Two,
        Three,
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
        Ace
    ]
}