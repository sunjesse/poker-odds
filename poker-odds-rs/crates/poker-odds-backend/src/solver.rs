use dashmap::DashMap;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;
use strum_macros::EnumIter;

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
    fn from_char(c: char) -> Self {
        match c {
            'c' => Suits::Clubs,
            'h' => Suits::Hearts,
            's' => Suits::Spades,
            'd' => Suits::Diamonds,
            _ => panic!("not a valid char"),
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
    Ace = 14,
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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Card {
    value: Value,
    suit: Suits,
    idx: usize,
}

impl Card {
    fn new(value: Value, suit: Suits) -> Self {
        let mut _idx = value as usize * 4 - 8;
        for (i, s) in [Suits::Clubs, Suits::Hearts, Suits::Spades, Suits::Diamonds]
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

    fn from_string(s: String) -> Self {
        let s: Vec<u8> = s.chars().map(|x| x as u8).collect();
        let value: u8 = match s[0] {
            65 => 14,
            75 => 13,
            81 => 12,
            74 => 11,
            84 => 10,
            50..=57 => s[0] - 48,
            _ => panic!("Not a valid value"),
        };
        let suit: Suits = Suits::from_char(s[1] as char);
        Self::new(Value::from(value), suit)
    }
}

#[derive(Debug, Clone)]
struct Hand {
    hole: (Card, Card),
    hole_b: u64,
    memo: HashMap<u64, Rank>,
    kicker: u32,
}

impl Hand {
    fn new(hole: (Card, Card)) -> Self {
        Hand {
            hole: hole,
            hole_b: 1 << hole.0.idx | 1 << hole.1.idx,
            memo: HashMap::new(),
            kicker: 0,
        }
    }

    fn rank(&mut self, board: &u64) -> Rank {
        let cards_key: u64 = self.hole_b | *board;

        if self.memo.contains_key(&cards_key) {
            return self.memo[&cards_key];
        }

        let mut _rank: Rank = Rank::HighCard;

        // TODO [optimization]:
        // The lower down the if-else statement,
        // the more likely the hand is. We are doing quite
        // a bit of branching here. TODO: Reduce amount of branching
        // needed?

        if self.is_royal_flush(&cards_key) {
            _rank = Rank::RoyalFlush;
        } else if self.is_straight_flush(&cards_key) {
            _rank = Rank::StraightFlush;
        } else if self.is_quads(&cards_key) {
            _rank = Rank::Quads;
        } else if self.is_fullhouse(&cards_key) {
            _rank = Rank::FullHouse;
        } else if self.is_flush(&cards_key) {
            _rank = Rank::Flush;
        } else if self.is_straight(&cards_key) {
            _rank = Rank::Straight;
        } else if self.is_three_of_a_kind(&cards_key) {
            _rank = Rank::Trips;
        } else if self.is_two_pair(&cards_key) {
            _rank = Rank::TwoPair;
        } else if self.is_pair(&cards_key) {
            _rank = Rank::Pair;
        } else {
            // _rank is Rank::HighCard.
            self.compute_kicker_for_high_card(&cards_key);
        }
        self.memo.insert(cards_key, _rank);
        _rank
    }

    fn is_royal_flush(&self, cards: &u64) -> bool {
        // mask := cards in a royal flush of suit clubs. shift left for next suit.
        let mut mask: u64 = 1 << 32 | 1 << 36 | 1 << 40 | 1 << 44 | 1 << 48;
        (0..4).fold(false, |acc, x| {
            mask <<= x;
            acc | ((mask & *cards) == mask)
        })
    }

    fn is_straight_flush(&mut self, cards: &u64) -> bool {
        // start at king high straight flush of suit club.
        // no need to check royal flush as we check that before.
        let mut mask: u64 = 1 << 28 | 1 << 32 | 1 << 36 | 1 << 40 | 1 << 44;
        let aces: u64 = 1 << 48 | 1 << 49 | 1 << 50 | 1 << 51;

        for i in 0..9 {
            for sh in 0..4 {
                let valid: bool = mask & *cards == mask;
                if (i < 8 && valid)
                    || (i == 8 && valid && ((*cards & aces) & (1 << (48 + sh)) != 0))
                {
                    self.kicker = 13 - i as u32;
                    return true;
                }
                mask <<= 1;
            }
            // go to next largest straight flush
            mask >>= 8;
        }
        false
    }

    fn is_quads(&mut self, cards: &u64) -> bool {
        let mut mask: u64 = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        let mut tmp: u32 = 0;
        for i in 0..13 {
            if mask & *cards == mask {
                tmp = 14 - i;
            }
            mask >>= 4;
        }

        if tmp == 0 {
            return false;
        }

        mask = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        for i in 0..13 {
            if i + tmp != 14 && mask & *cards != 0 {
                self.kicker = tmp * 100 + 14 - i;
                return true;
            }
            mask >>= 4;
        }
        false
    }

    fn is_fullhouse(&mut self, cards: &u64) -> bool {
        let mut mask: u64 = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        let mut tmp: u32 = 0;

        for i in 0..13 {
            if (mask & *cards).count_ones() == 3 {
                tmp = 14 - i;
                break;
            }
            mask >>= 4;
        }

        if tmp == 0 {
            return false;
        }

        mask = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        for i in 0..13 {
            // not the three of a kind
            if i + tmp != 14 {
                if (mask & *cards).count_ones() >= 2 {
                    self.kicker = tmp * 100 + 14 - i;
                    return true;
                }
            }
            mask >>= 4;
        }
        false
    }

    fn is_flush(&mut self, cards: &u64) -> bool {
        // start with clubs
        let mut mask: u64 = (0..52).step_by(4).fold(0, |acc, x| acc | (1 << x));
        for _ in 0..4 {
            let m: u64 = mask & *cards;
            if m.count_ones() >= 5 {
                // this won't return the exact highest card value, but its a monotonic
                // function and we save some instructions by avoiding needing to call %
                // to compute exact value.
                self.kicker = 64 - m.leading_zeros();
                return true;
            }
            mask <<= 1;
        }
        false
    }

    fn is_straight(&mut self, cards: &u64) -> bool {
        let mut key_bin: u16 = 0;
        // the following is all twos
        let mut repr: u64 = 1 | 1 << 1 | 1 << 2 | 1 << 3;

        for i in 0..13 {
            if *cards & repr != 0 {
                key_bin |= 1 << (i + 1);
                // if is ace
                if i == 12 {
                    key_bin |= 1;
                }
            }
            repr <<= 4;
        }

        let mut mask: u16 = 1 << 14 | 1 << 13 | 1 << 12 | 1 << 11 | 1 << 10;

        for i in 0..11 {
            if mask & key_bin == mask {
                self.kicker = 14 - i;
                return true;
            }
            mask >>= 1;
        }
        false
    }

    fn is_three_of_a_kind(&mut self, cards: &u64) -> bool {
        // this assumes its not a full house
        let mut mask: u64 = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        let mut tmp: u32 = 0;
        let mut count: usize = 0;

        for i in 0..13 {
            if (mask & *cards).count_ones() == 3 {
                tmp = 14 - i;
                count += 1;
                break;
            }
            mask >>= 4;
        }

        if count == 0 {
            return false;
        }

        mask = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        for i in 0..13 {
            if mask & *cards != 0 {
                tmp = tmp * 100 + 14 - i;
                count += 1;
            }
            if count == 3 {
                self.kicker = tmp;
                return true;
            }
            mask >>= 4;
        }
        false
    }

    fn is_two_pair(&mut self, cards: &u64) -> bool {
        let mut mask: u64 = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        let mut tmp: u32 = 0;
        let mut count: usize = 0;

        // find the two pair first
        for i in 0..13 {
            if (mask & *cards).count_ones() == 2 {
                tmp = tmp * 100 + 14 - i;
                count += 1;
            }
            mask >>= 4;
        }

        if count < 2 {
            return false;
        }

        // then find the kicker
        mask = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        for i in 0..13 {
            if mask & *cards != 0 {
                self.kicker = tmp * 100 + 14 - i;
                return true;
            }
            mask >>= 4;
        }
        false
    }

    fn is_pair(&mut self, cards: &u64) -> bool {
        let mut mask: u64 = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        let mut tmp: u32 = 0;
        let mut count: usize = 0;

        for i in 0..13 {
            if (mask & *cards).count_ones() == 2 {
                tmp = tmp * 100 + 14 - i;
                count += 1;
                break;
            }
            mask >>= 4;
        }

        if count == 0 {
            return false;
        }

        mask = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        for i in 0..13 {
            if mask & *cards != 0 {
                tmp = tmp * 100 + 14 - i;
                count += 1;
            }
            if count == 4 {
                self.kicker = tmp;
                return true;
            }
            mask >>= 4;
        }
        false
    }

    fn compute_kicker_for_high_card(&mut self, cards: &u64) {
        let mut mask: u64 = 1 << 51 | 1 << 50 | 1 << 49 | 1 << 48;
        let mut tmp: u32 = 0;
        let mut count: usize = 0;

        for i in 0..13 {
            if (mask & *cards).count_ones() == 1 {
                tmp = tmp * 100 + 14 - i;
                count += 1;
            }

            if count == 5 {
                self.kicker = tmp;
                break;
            }
            mask >>= 4;
        }
    }

    fn from_string(s: String) -> Self {
        let (h1, h2) = s.split_at(2);
        Hand::new((
            Card::from_string(h1.to_string()),
            Card::from_string(h2.to_string()),
        ))
    }
}

#[derive(Debug, Clone)]
struct Game {
    hero_pos: usize,
    hands: Vec<Hand>,
}

impl Game {
    pub fn new(hero_pos: usize, hands: Vec<Hand>) -> Self {
        Game { hero_pos, hands }
    }
}

#[derive(Debug, Clone)]
struct BitSet {
    s: u64,
    length: usize,
}

impl BitSet {
    fn new() -> Self {
        BitSet { s: 0, length: 0 }
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

    fn add_board(&mut self, board: &u64) {
        self.length += ((*board).count_ones() - (*board & self.s).count_ones()) as usize;
        self.s |= *board;
    }
}

#[derive(Debug, Clone)]
struct Brancher {
    game: Game,
    hero: Hand,
    drawn: BitSet,
    board: u64,
    memo: Arc<DashMap<u64, f32>>,
}

impl Brancher {
    fn new(game: Game, board: u64, memo: Arc<DashMap<u64, f32>>) -> Self {
        let hero = game.hands[game.hero_pos].clone();
        let mut drawn = BitSet::new();

        for hand in game.hands.iter() {
            drawn.add(hand.hole.0.idx);
            drawn.add(hand.hole.1.idx);
        }

        drawn.add_board(&board);

        Brancher {
            game,
            hero,
            drawn,
            board,
            memo,
        }
    }

    fn branch(&mut self, board: &mut u64) -> f32 {
        if let Some(val) = self.memo.get(&self.drawn.s) {
            return *val;
        }

        if board.count_ones() == 5 {
            let hero_rank = self.hero.rank(board);
            let hero_kicker = self.hero.kicker;

            let beats_all = self
                .game
                .hands
                .iter_mut()
                .enumerate()
                .filter(|&(i, _)| i != self.game.hero_pos)
                .all(|(_, hand)| {
                    let v = hand.rank(board);
                    hero_rank > v || (hero_rank == v && hero_kicker > hand.kicker)
                });
            let val: f32 = if beats_all { 1. } else { 0. };
            self.memo.insert(self.drawn.s, val);
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
        self.memo.insert(self.drawn.s, pb);
        pb
    }

    fn branch_parallel(&self, nthreads: usize) -> f32 {
        println!("Running on {:} threads.", nthreads);

        let step: usize = 52 / nthreads;
        let chunks: Vec<(usize, usize)> = (0..52)
            .step_by(step)
            .map(|s| (s, (s + step).min(52)))
            .collect();

        let handles: Vec<_> = chunks
            .into_iter()
            .map(|(s, e)| {
                let mut local_brancher = self.clone();
                thread::spawn(move || {
                    let mut pb: f32 = 0.;
                    let mut board: u64 = local_brancher.board;
                    for i in s..e {
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

        let mut sum_pb: f32 = 0.;
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

    fn compute_equity(&mut self) -> f32 {
        /*
        Run on one thread if 4 cards are
        already on the board to avoid overhead
        of copying and moving onto threads.
        */
        if let Some(val) = self.memo.get(&self.drawn.s) {
            println!("[Cached] Equity is {:}.", *val);
            return *val;
        }

        let nthreads: usize = 8;
        let p: f32;

        if self.board.count_ones() >= 4 {
            let mut board: u64 = self.board.clone();
            p = self.branch(&mut board);
        } else {
            p = self.branch_parallel(nthreads);
            self.memo.insert(self.drawn.s, p);
        }
        println!("Equity is {:}.", p);
        p
    }
}

pub struct Solver {
    memo: Arc<DashMap<u64, f32>>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            memo: Arc::new(DashMap::new()),
        }
    }

    pub fn solve(&self, hands: &Vec<String>, bd: &String) -> f32 {
        let mut hs: Vec<Hand> = Vec::new();

        for hand in hands {
            hs.push(Hand::from_string(hand.to_string()));
        }

        let bd: Vec<char> = bd.chars().collect();
        let mut board: u64 = 0;
        for chunk in bd.chunks(2) {
            let c: String = chunk.iter().collect();
            let card: Card = Card::from_string(c);
            board |= 1 << card.idx;
        }

        let game = Game::new(0, hs);
        let mut brancher = Brancher::new(game, board, self.memo.clone());
        println!("START: {:?}", SystemTime::now());
        let p: f32 = brancher.compute_equity();
        println!("END: {:?}", SystemTime::now());
        p
    }
}

fn pop_extra_characters(s: &mut String) {
    while matches!(s.chars().last(), Some('\n')) {
        s.pop();
    }
}

#[allow(dead_code)]
pub fn parse_input_and_solve() {
    /*
    By threading & sharing memo table across threads,
    we get the following result on a board with 0 cards
    running on 8 threads:

        1 thread (Python): 60 seconds
        1 thread (Rust): 60 seconds
        8 threads - Without sharing memo: 60 seconds
        8 threads - With sharing memo: 16 seconds.
        8 threads with opt-level 3 + sharing memo: 5 seconds.
        8 threads w/ opt l3 + sharing memo w/ rwlock: < 3 seconds
        8 threads w/ opt l3 + memo as dashmap: < 1 seconds
        The row above + all computations binary - remove heap allocation during Hand.rank call: < 400 ms
    */

    let solution: Solver = Solver::new();

    loop {
        println!("# active players [0 to exit]:");
        let mut nplayers = String::new();
        io::stdin()
            .read_line(&mut nplayers)
            .expect("Failed to get console input");
        let nplayers = nplayers.trim().parse::<i32>().expect("Failed to parse int");
        if nplayers == 0 {
            break;
        }

        let mut hs: Vec<String> = Vec::new();

        for i in 0..nplayers {
            if i == 0 {
                println!("Your starting hand: ");
            } else {
                println!("Opponent {} hand: ", i);
            }
            let mut x = String::new();
            io::stdin()
                .read_line(&mut x)
                .expect("Failed to get console input");

            pop_extra_characters(&mut x);
            hs.push(x);
        }

        println!("Board: ");
        let mut bd: String = String::new();
        io::stdin()
            .read_line(&mut bd)
            .expect("Failed to get console input");
        pop_extra_characters(&mut bd);
        solution.solve(&hs, &bd);
    }
}
