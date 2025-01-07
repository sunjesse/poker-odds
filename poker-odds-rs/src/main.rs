use rand::seq::SliceRandom;
use rand::thread_rng;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::collections::HashMap;
use std::thread;
use std::time::SystemTime;

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

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        match value {
            2 => Value::Two,
            3 => Value::Three,
            4 => Value::Four,
            5 => Value::Five,
            6 => Value::Six,
            7 => Value::Seven,
            8 => Value::Eight,
            9 => Value::Nine,
            10 => Value::Ten,
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            14 => Value::Ace,
            _ => panic!("Invalid card value"),
        }
    }
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
    memo: HashMap<u64, Rank>,
    kicker: u32,
}

impl Hand {
    fn new(hole: (Card, Card)) -> Self {
        Hand {
            hole: hole,
            memo: HashMap::new(),
            kicker: 0, 
        } 
    }

    fn rank(&mut self, board: &u64) -> Rank {
        let mut cards_key: u64 = 1 << self.hole.0.idx | 1 << self.hole.1.idx | *board; 

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

        /*
        Backwards Conversion:
             suit = idx % 4;
             value = (idx - (idx%4))/4 + 2
        */
        // Get state of board from binary repr.
        for i in 0..52 {
            if *board >> i & 1 == 1 {
                let value: u8 = (i - (i%4))/4 + 2;
                let suit: Suits = match i % 4 {
                    0 => Suits::Clubs,
                    1 => Suits::Hearts,
                    2 => Suits::Diamonds,
                    3 => Suits::Spades,
                    _ => unreachable!(),
                }; 
                suits.entry(suit)
                    .or_insert(Vec::new())
                    .push(value);
                *_values.entry(value).or_insert(0) += 1;
            }
        }


        let mut values: Vec<_> = _values.into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        values.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

        let mut _rank: Rank = Rank::HighCard;
        
        // TODO [optimization]: 
        // Can make all these computations bitwise on u64s to
        // avoid the need for creating HashMaps, vecs,
        // and other objects.

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
        } else if self.is_pair(&values) {
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
                        self.kicker = values[i+4] as u32;
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
        if values.len() >= 2 {
            if let (Some(x), Some(y)) = (values.last(), values.get(values.len() - 2)) {
                if y.1 >= 2 && x.1 >= 3 {
                    self.compute_kicker_as_best_five(2, &values);
                    return true;
                }
            }
        }
        false
    }

    fn is_flush(&mut self, suits: &HashMap<Suits, Vec<u8>>) -> bool {
        for (_, v) in suits.iter() {
            if v.len() >= 5 {
                self.kicker = *v.iter().max().unwrap() as u32;
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
                    self.kicker = keys[i+4] as u32;
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
        if values.len() >= 2 {
            if let (Some(x), Some(y)) = (values.last(), values.get(values.len() - 2)) {
                if x.1 == 2 && y.1 == 2 {
                    self.compute_kicker_as_best_five(3, &values);
                    return true;
                }
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
        let mut _kicker: u32 = 0;
        for i in 0..(values.len().min(ubound)) {
            _kicker *= 100;
            _kicker += values[values.len()-i-1].0 as u32;
        }
        self.kicker = _kicker;
    }
}


#[derive(Debug, Clone)]
struct Game {
    nplayers: usize,
    hero_pos: usize,
    hands: Vec<Hand>,
}

impl Game {
    pub fn new(
        nplayers: usize,
        hero_pos: usize,
        hands: Vec<Hand>,
    ) -> Self {
        Game {
            nplayers,
            hero_pos,
            hands,
        }
    }
}


#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
struct Brancher {
    game: Game,
    hero: Hand,
    drawn: BinarySet,
    board: u64,
    memo: HashMap<u64, f32>,
}

impl Brancher {
    fn new(game: Game, board: u64) -> Self {
        let hero = game.hands[game.hero_pos].clone();

        let mut drawn = BinarySet::new();

        for hand in game.hands.iter() {
            drawn.add(hand.hole.0.idx);
            drawn.add(hand.hole.1.idx);
        }

        drawn.s |= board;

        Brancher {
            game,
            hero,
            drawn,
            board,
            memo: HashMap::new(),
        }
    }

    fn branch(&mut self, board: &mut u64) -> f32 {
        if self.memo.contains_key(board) {
            return self.memo[board];
        }
    
        if board.count_ones() == 5 {
            let hero_rank = self.hero.rank(board);
            let hero_kicker = self.hero.kicker;

            let beats_all = self.game.hands.iter_mut()
                .enumerate()
                .filter(|&(i, _)| i != self.game.hero_pos)
                .all(|(i, hand)| {
                    let v = hand.rank(board);
                    hero_rank > v || (hero_rank == v && hero_kicker > hand.kicker)
                });
            let val = if beats_all { 1. } else { 0. };
            self.memo.insert(*board, val);
            return val;    
        }

        let mut pb: f32 = 0.;
        for i in 0..52 {
            if !self.drawn.contains(i) {
                self.add_to_end_of_board(i, board);
                pb += self.branch(board);
                self.remove_from_end_of_board(i, board);
            }
        }
        pb /= (52 - self.drawn.len()) as f32;
        self.memo.insert(*board, pb);
        pb
    }

    fn branch_parallel(&self, nthreads: usize) -> f32 {
        /*
        Currently, multithreading doesn't really improve performance.
        I suspect it is because we are chunking by first card only,
        and the memoization is not shared across threads. We could
        share it although that may result in performance hit due
        to mutex? 
        
        We should try chunking via first 3 cards, < 52 choose 3 ~= 22k
        combinations.
        */
        println!("Running on {:} threads.", nthreads); 

        let chunk_size: usize = 52 / nthreads;
        let chunks: Vec<(usize, usize)> = (0..52)
            .step_by(chunk_size)
            .map(|start| (start, (start + chunk_size).min(52)))
            .collect();

        let handles: Vec<_> = chunks
            .into_iter()
            .map(|(start, end)| {
                let mut local_brancher = self.clone();
                thread::spawn(move || {
                    let mut pb = 0.;
                    let mut board: u64 = local_brancher.board;
                    println!("Spawning on thread {:?}...", thread::current().id());
                    for i in start..end {
                        if !local_brancher.drawn.contains(i) {
                            local_brancher.add_to_end_of_board(i, &mut board);
                            pb += local_brancher.branch(&mut board);
                            local_brancher.remove_from_end_of_board(i, &mut board);
                        }
                    }

                    pb
                })
            })
            .collect();

        let mut sum_pb = 0.;
        for h in handles {
            sum_pb += h.join().unwrap();
        }

        sum_pb / (52 - self.drawn.len()) as f32
    }

    fn add_to_end_of_board(&mut self, card_idx: usize, board: &mut u64) {
        self.drawn.add(card_idx);
        *board |= 1 << card_idx;
    }
    
    fn remove_from_end_of_board(&mut self, card_idx: usize, board: &mut u64) {
        self.drawn.remove(card_idx);
        *board -= 1 << card_idx;
    }

}



fn main() {
    // pre-flop still takes 61 seconds ish on 8 threads?
    let mut board: u64 = 1 << 3 | 1 << 4 | 1 << 5; //| 1 << 6; // | 1 << 7;
    let h1 = Card::new(Value::Two, Suits::Hearts);
    let h2 = Card::new(Value::Two, Suits::Diamonds);
    let mut hand = Hand::new((h1, h2));
    let mut vh = Hand::new((Card::new(Value::Three, Suits::Clubs), Card::new(Value::Three, Suits::Hearts)));
    println!("{:?}", hand.rank(&board));
    println!("{:?}", vh.rank(&board));
    let vec_hand = Vec::from([hand, vh]);
    let game = Game::new(2, 0, vec_hand);

    println!("START: {:?}", SystemTime::now());
    let mut brancher = Brancher::new(game, board);
    let nthreads: usize = 16;
    println!("Equity is {:?}", brancher.branch_parallel(nthreads));
    println!("END: {:?}", SystemTime::now());
}
