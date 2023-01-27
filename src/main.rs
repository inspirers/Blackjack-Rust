fn main() {
    println!("Hello, world!");
    let deck = generateDeck();
    println!("{:?}",deck);
}
#[derive(Copy, Clone,Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}
// #[macro_use] extern crate cute;
fn generateDeck() -> Vec<Card> {
    let Suit: Vec<Suit> = vec![Suit::Clubs,Suit::Diamonds,Suit::Hearts, Suit::Spades];
    let Rank: Vec<Rank> = vec![
        Rank::Numeric(2),
        Rank::Numeric(3),
        Rank::Numeric(4),
        Rank::Numeric(6),
        Rank::Numeric(7),
        Rank::Numeric(8),
        Rank::Numeric(9),
        Rank::Numeric(10),
        Rank::Ace,
    ];
    let mut cards: Vec<Card> = Vec::new();
    for suit in &Suit {
        for rank in &Rank {
            cards.push(Card {suit: *suit, rank: *rank});
        }
    }
    // cards.push(Card {suit: Suit::Clubs, rank: Rank::Ace});
    cards


//     let v2 = (1..10).filter(|x| x % 2 == 0).collect::<Vec<u32>>();
//     let test = Card {suit: Suit::Clubs, rank: Rank::Jack};
//     let test2 = vec![0..10];
// let vector = c![Card {suit: Suit::suits, rank: Rank::ranks}, for suits in Suit, for ranks in Rank];
}
#[derive(PartialEq, PartialOrd,Copy,Clone,Debug)]

enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
// use std::fmt;
#[derive(PartialEq, PartialOrd,Copy,Clone,Debug)]
enum Rank {
    Numeric(i32),
    Jack,
    Queen,
    King,
    Ace,
}
