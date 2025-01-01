use rand::seq::SliceRandom;
use rand::thread_rng;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Rank {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    Trips = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    Quads = 7,
    StraightFlush = 8,
    RoyalFlush = 9,
}


#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
enum Suits {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

impl Suits {
    fn to_char(&self) -> char {
        match self {
            Suits::Clubs => 'c',
            Suits::Hearts => 'h',
            Suits::Spades => 's',
            Suits::Diamonds => 'd',
        }
    }    
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumIter)]
enum Value {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14
}


#[derive(Debug)]
struct Card {
    value: Value,
    suit: Suits,
    _idx: Option<usize>,
}

impl Card {
    fn new(value: Value, suit: Suits) -> Self {
        Card {
            value,
            suit,
            _idx: None,
        }
    }

    fn idx(&mut self) -> usize {
        if self._idx.is_none() {
            let mut _idx = self.value as usize * 4 - 8;
            for (i, s) in [
                Suits::Clubs,
                Suits::Hearts,
                Suits::Spades,
                Suits::Diamonds,
            ]
            .iter()
            .enumerate()
            {
                if self.suit == *s {
                    _idx += i;
                    break;
                }
            }
            self._idx = Some(_idx);
        }
        self._idx.unwrap() 
    }
}


#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut deck = Deck {
            cards: Vec::new(),
        };
        
        for value in Value::iter() {
            for suit in Suits::iter() {
                deck.cards.push(Card::new(value, suit));
            }
        }
        deck.shuffle();
        println!("{:?}", deck);
        deck
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
    fn len(&self) -> usize {
        self.cards.len()
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

fn main() {
    if Rank::Pair < Rank::TwoPair {
        println!("True");
    }
    println!("{}", Suits::Clubs.to_char()); 
    let mut card = Card::new(Value::Two, Suits::Hearts);
    let mut deck = Deck::new();
    println!("IDX {}", card.idx());
}
