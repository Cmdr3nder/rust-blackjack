use std::rand;

pub mod rank;
pub mod suit;
pub mod card;

fn main() {
    let mut deck: Vec<card::Card> = create_deck();
    shuffle_deck(&mut deck);
    for card in deck.iter() {
        println!("{}", card);
    }
}

fn shuffle_deck(deck: &mut Vec<card::Card>) {
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

fn create_deck() -> Vec<card::Card> {
    let mut deck: Vec<card::Card> = Vec::new();

    for suit in suit::suits_to_vector().iter() {
        for rank in rank::ranks_to_vector().iter() {
            deck.push(card::Regular(*suit, *rank));
        }
    }

    deck
}