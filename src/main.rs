use std::rand;
use std::fmt;

enum Suit {
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

enum Rank {
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
            Ace => write!(f, "Ace"),
            Two => write!(f, "2"),
            Three => write!(f, "3"),
            Four => write!(f, "4"),
            Five => write!(f, "5"),
            Six => write!(f, "6"),
            Seven => write!(f, "7"),
            Eight => write!(f, "8"),
            Nine => write!(f, "9"),
            Ten => write!(f, "10"),
            Jack => write!(f, "Jack"),
            Queen => write!(f, "Queen"),
            King => write!(f, "King")
        }
    }
}

enum Card {
    Regular(Suit, Rank),
    Joker,
    Blank
}

impl fmt::Show for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Regular(suit, rank) => write!(f, "{} of {}", rank, suit),
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

fn main() {
    let mut deck: Vec<Card> = create_deck();
    shuffle_deck(&mut deck);
    for card in deck.iter() {
        println!("{}", card);
    }
}

fn shuffle_deck(deck: &mut Vec<Card>) {
    for _x in range(0u, 1000u) {
        //Generate index
        let index = random_in_range(0u, deck.len() - 1);
        //Move to end
        match deck.swap_remove(index) {
            Some(card) => deck.push(card),
            None => println!("Something is wrong with our deck!")
        }
    }
}

fn random_in_range(x: uint, y: uint) -> uint{ //Inclusive
    let diff = y - x;
    (rand::random::<uint>() % (diff + 1)) + x
}

fn create_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();

    let suit_array = [
        Clubs,
        Diamonds,
        Spades,
        Hearts
    ];

    let rank_array = [
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
    ];

    for suit in suit_array.iter() {
        for rank in rank_array.iter() {
            deck.push(Regular(*suit, *rank));
        }
    }

    deck
}