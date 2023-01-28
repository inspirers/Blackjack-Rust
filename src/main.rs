use std::fmt; // Import `fmt`
fn main() {
    println!("Hello, world!");
    let deck: Deck = generate_deck();
    println!("{}", deck);
    // println!(
    //     "{}",
    //     Card {
    //         suit: Suit::Clubs,
    //         rank: Rank::Ace
    //     }
    // );
    // println!(
    //     "{}",
    //     Card {
    //         suit: Suit::Clubs,
    //         rank: Rank::Numeric(7)
    //     }
    // );
    // println!("{}", Rank::Ace);
    // println!("{}", Rank::Numeric(5));
    // println!("{}", Suit::Spades);
}
#[derive(Copy, Clone)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card {} {}", self.suit, self.rank)
    }
}
struct Deck(pub Vec<Card>);

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, card| {
            result.and_then(|_| writeln!(f, "{} of {}", card.rank, card.suit))
        })
    }
}


#[macro_use]
extern crate cute;
fn generate_deck() -> Deck {
    let suit_list: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    let rank_list: Vec<Rank> = vec![
        Rank::Numeric(2),
        Rank::Numeric(3),
        Rank::Numeric(4),
        Rank::Numeric(5),
        Rank::Numeric(6),
        Rank::Numeric(7),
        Rank::Numeric(8),
        Rank::Numeric(9),
        Rank::Numeric(10),
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
    // let mut cards: Vec<Card> = Vec::new();
    // for suit in &Suit {
    //     for rank in &Rank {
    //         cards.push(Card {suit: *suit, rank: *rank});
    //     }
    // }
    // cards.push(Card {suit: Suit::Clubs, rank: Rank::Ace});
    let deck = Deck(
        c![Card {suit: *suits, rank: ranks}, for suits in &suit_list, for ranks in rank_list]);
    deck

}
#[derive(PartialEq, PartialOrd, Copy, Clone)]

enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Spades => write!(f, "♠"),
            Suit::Clubs => write!(f, "♣"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
enum Rank {
    Numeric(i32),
    Jack,
    Queen,
    King,
    Ace,
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rank::Numeric(rank) => write!(f, "{:?}", rank),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
            Rank::Ace => write!(f, "Ace"),
        }
    }
}
