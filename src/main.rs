use std::rand;

enum Suit {
    Clubs,
    Diamonds,
    Spades,
    Hearts
}

impl Suit {
    fn to_string(&self) -> String {
        let x = match *self {
            Clubs => "Clubs",
            Diamonds => "Diamonds",
            Spades => "Spades",
            Hearts => "Hearts"
        };

        x.to_string()
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

impl Rank {
    fn to_string(&self) -> String {
        let x = match *self {
            Ace => "Ace",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "Jack",
            Queen => "Queen",
            King => "King"
        };

        x.to_string()
    }
}

enum Card {
    Regular(Suit, Rank),
    Joker,
    Blank
}

impl Card {
    fn to_string(&self) -> String {
        match *self {
            Regular(suit, rank) => rank.to_string() + " of " + suit.to_string(),
            Joker => "Joker".to_string(),
            Blank => "Blank Card".to_string()
        }
    }

    fn print(&self) {
        println!("{}", self.to_string());
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
        card.print();
    }
}

fn shuffle_deck(deck: &mut Vec<Card>) {
    for _x in range(0u, 1000u) {
        //Generate index
        let index = random_in_range(0u, deck.len() - 1);
        //Move to end
        match (*deck).swap_remove(index) {
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