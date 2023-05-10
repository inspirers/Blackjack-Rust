use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{thread, time};

fn main() {
    let mut deck: Deck = generate_deck();
    let mut player_hand: Deck = Deck(Vec::new());
    let mut bank_hand: Deck = Deck(Vec::new());

    deck = shuffle(deck);

    println!("Welcome to the game.");
    println!("Your current score: {}", calculate_score(&player_hand));

    (deck, player_hand) = player_draw(deck, player_hand);
    bank_hand = bank_draw(deck, bank_hand);
    winner(bank_hand, player_hand);
}
fn winner(bank_hand: Deck, player_hand: Deck) {
    let player_score = calculate_score(&player_hand);
    let bank_score = calculate_score(&bank_hand);

    if player_score > 21 {
        println!("Your score: Bust ({})", player_score);
    } else {
        println!("Your score: {}", player_score);
    }
    println!("Bank score: {}", bank_score);

    if player_score > 21 {
        println!("Winner: Bank");
    } else if player_score >= bank_score {
        println!("Winner: Player");
    } else {
        println!("Winner: Bank");
    }
}

fn bank_draw(mut deck: Deck, mut bank_hand: Deck) -> Deck {
    let mut bank_score = 0;
    while bank_score <= 16 {
        (deck, bank_hand) = draw_card(deck, bank_hand);
        bank_score = calculate_score(&bank_hand);

        println!("Bank current score: {}", bank_score);
        println!("Deck:\n{}", bank_hand);
        thread::sleep(time::Duration::from_secs(1));
    }
    bank_hand
}

fn player_draw(mut deck: Deck, mut hand: Deck) -> (Deck, Deck) {
    println!("Draw a card? [y/n]");
    let mut player_score;
    let mut user_input = std::io::stdin().lines().next().unwrap().unwrap();

    while user_input.eq("y") {
        (deck, hand) = draw_card(deck, hand);
        player_score = calculate_score(&hand);
        println!("Your current score: {}", player_score);
        println!("Deck:\n{}", hand);
        if player_score > 21 {
            break;
        }
        println!("Draw a card? [y/n]");
        user_input = std::io::stdin().lines().next().unwrap().unwrap();
    }
    (deck, hand)
}

fn draw_card(mut deck: Deck, mut hand: Deck) -> (Deck, Deck) {
    hand.0.push(deck.0[0]);
    deck.0.remove(0);
    (deck, hand)
}

fn shuffle(mut deck: Deck) -> Deck {
    deck.0.shuffle(&mut thread_rng());
    println!("{}", deck);

    deck
}

fn calculate_score(deck: &Deck) -> i32 {
    let mut score = 0;
    for card in &deck.0 {
        score += score_rank(*card);
    }
    if score > 21 {
        score -= 10 * rank_occurence(deck, Rank::Ace)
    }

    score
}
const fn score_rank(card: Card) -> i32 {
    match card.rank {
        Rank::Numeric(x) => x,
        Rank::Ace => 11,
        _ => 10, // Jack, Queen, King
    }
}

fn rank_occurence(deck: &Deck, rank: Rank) -> i32 {
    let mut tot_ranks = 0;
    for card in &deck.0 {
        if card.rank == rank {
            tot_ranks += 1;
        }
    }
    tot_ranks
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

    Deck(c![Card {suit: *suits, rank: *ranks}, for suits in &suit_list, for ranks in &rank_list])
}

struct Deck(pub Vec<Card>);

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, card| {
            result.and_then(|_| writeln!(f, "{} of {}", card.rank, card.suit))
        })
    }
}
#[derive(Copy, Clone)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Card {} {}", self.suit, self.rank)
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Rank::Numeric(rank) => write!(f, "{:?}", rank),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
            Rank::Ace => write!(f, "Ace"),
        }
    }
}
