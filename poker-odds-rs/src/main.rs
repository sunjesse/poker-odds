use rand::seq::SliceRandom;
use rand::thread_rng;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

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
    idx: usize,
}

impl Card {
    fn new(value: Value, suit: Suits) -> Self {
        let mut _idx = value as usize * 4 - 8;
        for (i, s) in [
            Suits::Clubs,
            Suits::Hearts,
            Suits::Spades,
            Suits::Diamonds,
        ]
        .iter()
        .enumerate()
        {
            if suit == *s {
                _idx += i;
                break;
            }
        }

        Card {
            value,
            suit,
            idx: _idx,
        }
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


#[derive(Debug, Clone)]
struct Hand {
    hole: (Card, Card),
    board: Rc<RefCell<Vec<Card>>>,
    memo: HashMap<u64, Rank>,
    kicker: u16,
}

impl Hand {
    fn new(hole: (Card, Card), board: Rc<RefCell<Vec<Card>>>) -> Self {
        Hand {
            hole: hole,
            board: board,
            memo: HashMap::new(),
            kicker: 0, 
        } 
    }

    fn rank(&mut self) -> Rank {
        let board = self.board.borrow();
        let mut cards_key: u64 = 1 << self.hole.0.idx | 1 << self.hole.1.idx | 
            board.iter().map(|card| 1 << card.idx).fold(0, |acc, x| acc | x);

        if self.memo.contains_key(&cards_key) {
            return self.memo[&cards_key];
        }

        let mut suits: HashMap<Suits, Vec<u8>> = HashMap::new();
        let mut _values: HashMap<u8, u8> = HashMap::new();
        
        suits.entry(self.hole.0.suit)
            .or_insert(Vec::new())
            .push(self.hole.0.value as u8);

        suits.entry(self.hole.1.suit)
            .or_insert(Vec::new())
            .push(self.hole.1.value as u8);

        *_values.entry(self.hole.0.value as u8)
            .or_insert(0) += 1;

        *_values.entry(self.hole.1.value as u8)
            .or_insert(0) += 1;

        for card in board.iter() {
            suits.entry(card.suit)
                .or_insert(Vec::new())
                .push(card.value as u8);

            *_values.entry(card.value as u8).or_insert(0) += 1;  
        }

        // Release the immutable borrow of self
        drop(board);

        let mut values: Vec<_> = _values.into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        values.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

        let mut _rank: Rank = Rank::HighCard;

        if self.is_royal_flush(&suits) {
            _rank = Rank::RoyalFlush;
        } else if self.is_straight_flush(&suits) {
            _rank = Rank::StraightFlush;
        } else if self.is_quads(&values) {
            _rank = Rank::Quads;
        } else if self.is_fullhouse(&values) {
            _rank = Rank::FullHouse;
        } else if self.is_flush(&suits) {
            _rank = Rank::Flush;
        } else if self.is_straight(&values) {
            _rank = Rank::Straight;
        } else if self.is_three_of_a_kind(&values) {
            _rank = Rank::Trips;
        } else if self.is_two_pair(&values) {
            _rank = Rank::TwoPair;
        } else if self.is_two_pair(&values) {
            _rank = Rank::Pair;
        } else {
            // _rank is Rank::HighCard.
            self.compute_kicker_as_best_five(2, &values);
        }
        self.memo.insert(cards_key, _rank);
        _rank
    }

    fn is_royal_flush(&self, suits: &HashMap<Suits, Vec<u8>>) -> bool {
        for (suit, values) in suits.iter() {
            if values.len() >= 5 && [10, 11, 12, 13, 14].iter().all(|&item| values.contains(&item)) {
                return true;
            }
        }
        false
    }

    fn is_straight_flush(&mut self, suits: &HashMap<Suits, Vec<u8>>) -> bool {
        for (suit, values) in suits.iter() {
            if values.len() >= 5 {
                let mut vals: Vec<u8> = values.to_vec();
                vals.sort();
                if *vals.last().unwrap() == 14 {
                    vals.insert(0, 1);
                }
                for i in (0..(vals.len()-4)).rev() {
                    if vals[i+4] - vals[i] == 4 {
                        self.kicker = values[i+4] as u16;
                        return true;
                    }
                }
            }
        }
        false
    } 

    fn is_quads(&mut self, values: &Vec<(u8, u8)>) -> bool {
        if let Some(x) = values.last() {
            if x.1 == 4 {
                self.compute_kicker_as_best_five(2, &values);
                return true;
            }
        }
        false
    } 
    
    fn is_fullhouse(&mut self, values: &Vec<(u8, u8)>) -> bool {
        if let (Some(x), Some(y)) = (values.last(), values.get(values.len() - 2)) {
            if y.1 >= 2 && x.1 >= 3 {
                self.compute_kicker_as_best_five(2, &values);
                return true;
            }
        }
        false
    }

    fn is_flush(&mut self, suits: &HashMap<Suits, Vec<u8>>) -> bool {
        for (_, v) in suits.iter() {
            if v.len() >= 5 {
                self.kicker = *v.iter().max().unwrap() as u16;
                return true;
            }
        }
        false
    }

    fn is_straight(&mut self, values: &Vec<(u8, u8)>) -> bool {
        let mut keys: Vec<u8> = values.iter().map(|(k, v)| *k).collect();
        keys.sort();

        if let Some(last) = keys.last() {
            if *last == 14 {
                keys.insert(0, 1);
            }
        }

        if keys.len() >= 5 {
            for i in (0..(keys.len()-4).max(0)).rev() {
                if keys[i+4] == keys[i] + 4 {
                    self.kicker = keys[i+4] as u16;
                    return true;
                }
            } 
        }
        false
    }

    fn is_three_of_a_kind(&mut self, values: &Vec<(u8, u8)>) -> bool {
        if let Some(x) = values.last() {
            if x.1 == 3 {
                self.compute_kicker_as_best_five(3, &values);
                return true;
            }
        }
        false
    }

    fn is_two_pair(&mut self, values: &Vec<(u8, u8)>) -> bool {
        if let (Some(x), Some(y)) = (values.last(), values.get(values.len() - 2)) {
            if x.1 == 2 && y.1 == 2 {
                self.compute_kicker_as_best_five(3, &values);
                return true;
            }
        }
        false
    }

    fn is_pair(&mut self, values: &Vec<(u8, u8)>) -> bool {
        if let Some(x) = values.last() {
            if x.1 == 2 {
                self.compute_kicker_as_best_five(4, &values);
                return true;
            }
        }
        false
    }

    fn compute_kicker_as_best_five(&mut self, ubound: usize, values: &Vec<(u8, u8)>) {
        let mut _kicker: u16 = 0;
        for i in 0..(values.len().min(ubound)) {
            _kicker *= 100;
            _kicker += values[values.len()-i-1].0 as u16;
        }
        self.kicker = _kicker;
    }

    fn gt(&mut self, other: &mut Hand) -> bool {
        let self_rank = self.rank();
        let other_rank = (*other).rank();
        return self_rank > other_rank || (self_rank == other_rank && self.kicker > other.kicker);
    }
}


#[derive(Debug)]
struct Game {
    nplayers: usize,
    hero_pos: usize,
    hands: Rc<RefCell<Vec<Hand>>>,
    board: Rc<RefCell<Vec<Card>>>,
    deck: Rc<RefCell<Deck>>,
}


impl Game {
    pub fn new(nplayers: usize,
           hero_pos: usize,
           hands: Rc<RefCell<Vec<Hand>>>,
           board: Rc<RefCell<Vec<Card>>>,
           deck: Rc<RefCell<Deck>>) -> Self {
        
        Game {
            nplayers: nplayers,
            hero_pos: hero_pos,
            hands: hands,
            board: board,
            deck: deck,
        }
    }

    fn outs_one_street(&self) -> Option<Vec<Card>> {
        let deck = self.deck.borrow();

        if self.board.borrow().len() >= 5 {
            return None;
        }

        let mut outs: Vec<Card> = Vec::new();

        let mut hands = self.hands.borrow_mut();

        for card in deck.cards.iter() {
            // not pretty but it will do since we need to scope out mut and immut (in .rank()) borrows.
            self.board.borrow_mut().push(*card);
            let hero_rank = hands[self.hero_pos].rank();
            let hero_kicker = hands[self.hero_pos].kicker;
            let beats_all = hands.iter_mut()
                .enumerate()
                .filter(|&(i, _)| i != self.hero_pos)
                .all(|(i, hand)| {
                    let v = hand.rank();
                    hero_rank > v || (hero_rank == v && hero_kicker > hand.kicker)
                });
            if beats_all {
                outs.push(*card);
            }

            self.board.borrow_mut().pop();
        }

        Some(outs)
    }

    fn compute_odds(&self) -> f32 {
        if let Some(outs) = self.outs_one_street() {
            return outs.len() as f32 / self.deck.borrow().len() as f32;
        }
        0.
    }

}


#[derive(Debug)]
struct BinarySet {
    s: u64,
    length: usize,
}

impl BinarySet {
    fn new() -> Self {
        BinarySet {
            s: 0,
            length: 0,
        }
    }

    fn add(&mut self, idx: usize) {
        if !self.contains(idx) {
            self.s |= 1 << idx;
            self.length += 1;
        } 
    }

    fn remove(&mut self, idx: usize) {
        if self.contains(idx) {
            self.s -= 1 << idx;
            self.length -= 1;
        }
    }

    fn contains(&self, idx: usize) -> bool {
        (self.s >> idx) & 1 == 1
    }

    fn len(&self) -> usize {
        self.length
    }
}


#[derive(Debug)]
struct Brancher {
    game: Rc<RefCell<Game>>,
    hero: Rc<RefCell<Hand>>,
    drawn: BinarySet,
    memo: HashMap<u64, f32>,
}

impl Brancher {
    fn new(game: Rc<RefCell<Game>>) -> Self {
        let game_brw = game.borrow();

        let hero = game_brw.hands.borrow()[game_brw.hero_pos].clone();

        let mut drawn = BinarySet::new();
        
        for hand in game_brw.hands.borrow().iter() {
            drawn.add(hand.hole.0.idx);
            drawn.add(hand.hole.1.idx);
        }

        for card in game_brw.board.borrow().iter() {
            drawn.add(card.idx);
        }

        Brancher {
            game: game.clone(),
            hero: Rc::new(RefCell::new(hero)),
            drawn: drawn,
            memo: HashMap::new(), 
        }
    }

    fn branch(&mut self) -> f32 {
        let b: u64 =  self.binary_board();

        if self.memo.contains_key(&b) {
            return self.memo[&b];
        }

        let game_brw = self.game.borrow();
    
        if game_brw.board.borrow().len() == 5 {
            let mut hero_brw = self.hero.borrow_mut();
            let hero_rank = hero_brw.rank();
            let hero_kicker = hero_brw.kicker;

            let beats_all = game_brw.hands.borrow_mut().iter_mut()
                .enumerate()
                .filter(|&(i, _)| i != game_brw.hero_pos)
                .all(|(i, hand)| {
                    let v = hand.rank();
                    hero_rank > v || (hero_rank == v && hero_kicker > hand.kicker)
                });
            let val = if beats_all { 1. } else { 0. };
            self.memo.insert(b, val);
            return val;    
        }

        drop(game_brw);

        let mut pb: f32 = 0.;
        let ncards: usize = self.game.borrow().deck.borrow().len();

        // TODO: Fix this, don't be cloning... temporary fix for borrowing issues.
        let cards: Vec<_> = self.game.borrow().deck.borrow().cards.clone();
        for card in cards.iter() {
            if !self.drawn.contains(card.idx) {
                self.add_to_end_of_board(card);
                pb += self.branch();
                self.remove_from_end_of_board();
            }
        }
        pb /= (ncards - self.drawn.length) as f32;
        self.memo.insert(b, pb);
        pb
    }

    fn add_to_end_of_board(&mut self, card: &Card) {
        self.game.borrow_mut().board.borrow_mut().push(*card);
        self.drawn.add(card.idx);
    }
    
    fn remove_from_end_of_board(&mut self) {
        if let Some(card) = self.game.borrow_mut().board.borrow_mut().pop() {
            self.drawn.remove(card.idx);
        }
    }

    fn binary_board(&self) -> u64 {
        let b: u64 = self.game
                    .borrow()
                    .board
                    .borrow()
                    .iter()
                    .map(
                        |card| 
                        1 << card.idx)
                    .fold(0, |acc, x| acc | x);
        b
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
    board.push(Card::new(Value::Three, Suits::Spades));
    board.push(Card::new(Value::Two, Suits::Clubs));
    board.push(Card::new(Value::Seven, Suits::Diamonds));
    let h1 = Card::new(Value::Two, Suits::Hearts);
    let h2 = Card::new(Value::Two, Suits::Diamonds);
    let board_ref = Rc::new(RefCell::new(board));
    deck.append(card);
    let mut hand = Hand::new((h1, h2), board_ref.clone());
    let vh = Hand::new((Card::new(Value::Three, Suits::Clubs), Card::new(Value::Three, Suits::Hearts)), board_ref.clone());
    println!("{:?}", hand.rank());
    let vec_hand = Vec::from([hand, vh]);
    let hand_ref = Rc::new(RefCell::new(vec_hand));
    let deck_ref = Rc::new(RefCell::new(deck));
    let game = Game::new(2, 0, hand_ref, board_ref, deck_ref);

    println!("{:?} {:}", game.outs_one_street(), game.compute_odds());
    let game_ref = Rc::new(RefCell::new(game));
    let mut brancher = Brancher::new(game_ref);
    println!("Equity is {:?}", brancher.branch());
}
