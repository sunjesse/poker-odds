use rand::seq::SliceRandom;
use rand::thread_rng;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::collections::HashMap;

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


#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, EnumIter)]
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


#[derive(Debug, Clone, Copy)]
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
    
    fn append(&mut self, card: Card) {
        self.cards.push(card);
    }
}


#[derive(Debug)]
struct Hand<'a> {
    hole: (Card, Card),
    board: &'a mut Vec<Card>, // TODO: Switch to RefCell or Arc
    memo: HashMap<u64, Rank>,
    kicker: i32,
}

impl<'a> Hand<'a> {
    fn new(hole: (Card, Card), board: &'a mut Vec<Card>) -> Self {
        Hand {
            hole: hole,
            board: board,
            memo: HashMap::new(),
            kicker: 0, 
        } 
    }

    fn rank(&mut self) -> Rank {
        let mut cards_key: u64 = 1 << self.hole.0.idx() | 1 << self.hole.1.idx() | 
            self.board.iter_mut().map(|card| 1 << card.idx()).fold(0, |acc, x| acc | x);

        if self.memo.contains_key(&cards_key) {
            return self.memo[&cards_key];
        }

        let mut suits: HashMap<Suits, Vec<u8>> = HashMap::new();
        let mut _values: HashMap<u8, u8> = HashMap::new();
        
        suits.entry(self.hole.0.suit).or_insert(Vec::new()).push(self.hole.0.value as u8);
        suits.entry(self.hole.1.suit).or_insert(Vec::new()).push(self.hole.1.value as u8);
        *_values.entry(self.hole.0.value as u8).or_insert(0) += 1;
        *_values.entry(self.hole.1.value as u8).or_insert(0) += 1;

        for card in self.board.iter() {
            suits.entry(card.suit).or_insert(Vec::new()).push(card.value as u8);
            *_values.entry(card.value as u8).or_insert(0) += 1;  
        }

        let mut values: Vec<_> = _values.into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        values.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

        let mut _rank: Option<Rank> = None;

        if self.is_royal_flush(&suits) {
            _rank = Some(Rank::RoyalFlush);
        } else if self.is_straight_flush(&suits) {
            _rank = Some(Rank::StraightFlush);
        }
        
        println!("{:?}", suits);
        println!("{:?}", values);
        Rank::HighCard
    }

    fn is_royal_flush(&self, suits: &HashMap<Suits, Vec<u8>>) -> bool {
        for (suit, values) in suits.iter() {
            if values.len() >= 5 && [10, 11, 12, 13, 14].iter().all(|&item| values.contains(&item)) {
                return true;
            }
        }
        false
    }

    fn is_straight_flush(&self, suits: &HashMap<Suits, Vec<u8>>) -> bool {
        for (suit, values) in suits.iter() {
            if values.len() >= 5 {
                let mut vals: Vec<u8> = values.to_vec();
                vals.sort();
                if *vals.last().unwrap() == 14 {
                    vals.insert(0, 1);
                }
                for i in (0..(vals.len()-5)).rev() {
                    if vals[i+4] - vals[i] == 4 {
                        return true;
                    }
                }
            }
        }
        false
    } 
}

fn main() {
    if Rank::Pair < Rank::TwoPair {
        println!("True");
    }
    println!("{}", Suits::Clubs.to_char()); 
    let card = Card::new(Value::Two, Suits::Hearts);
    let mut deck = Deck::new();
    let mut board: Vec<Card> = Vec::new();
    board.push(Card::new(Value::Ten, Suits::Hearts));
    let h1 = Card::new(Value::Two, Suits::Hearts);
    let h2 = Card::new(Value::Two, Suits::Diamonds);
    deck.append(card);
    let mut hand = Hand::new((h1, h2), &mut board);
    println!("{:?}", hand.rank());
}
