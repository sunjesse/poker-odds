from enum import Enum
from typing import List, Tuple
from dataclasses import dataclass, field
import random
from collections import defaultdict
from functools import total_ordering


@total_ordering
class Rank(Enum):
    HIGH_CARD = 0
    PAIR = 1
    TWO_PAIR = 2
    TRIPS = 3
    STRAIGHT = 4
    FLUSH = 5
    FULL_HOUSE = 6
    QUADS = 7
    STRAIGHT_FLUSH = 8
    ROYAL_FLUSH = 9

    def __lt__(self, other):
        return self.value < other.value

    def __eq__(self, other):
        return self.value == other.value


class Suits(Enum):
    CLUBS = 'c' 
    HEARTS = 'h'
    SPADES = 's'
    DIAMONDS = 'd'


@total_ordering
class Value(Enum):
    TWO = 2
    THREE = 3
    FOUR = 4
    FIVE = 5
    SIX = 6
    SEVEN = 7
    EIGHT = 8
    NINE = 9
    TEN = 10
    JACK = 11
    QUEEN = 12
    KING = 13
    ACE = 14
    
    def __lt__(self, other):
        if isinstance(other, int):
            return self.value < other
        return self.value < other.value

    def __eq__(self, other):
        if isinstance(other, int):
            return self.value == other
        return self.value == other.value

    def __hash__(self):
        return hash(self.value)


class Constants:
    ROYAL_FLUSH_VALUES = {Value.TEN, Value.JACK, Value.QUEEN, Value.KING, Value.ACE}


@dataclass
class Card:
    value: Value
    suit: Suits

    def __post_init__(self):
        if isinstance(self.value, int):
            try:
                self.value = Value(self.value)
            except ValueError:
                raise ValueError(f"Invalid card value: {self.value}")
        
        if self.value not in Value:
            raise ValueError(f"Invalid card value: {self.value}")

        self._idx = None 
        
    @property
    def idx(self):
        if self._idx is None:
            _idx = self.value.value * 4 - 8 # sub 8 to start from 0
            for i, s in enumerate(Suits):
                if self.suit == s:
                    _idx += i
                    break
            self._idx = _idx
        return self._idx

    def __hash__(self):
        return hash((self.value.value, self.suit.value))

    def __str__(self):
        return str(self.value.value) + str(self.suit.value)

    def __repr__(self):
        return self.__str__()


@dataclass
class Deck:
    cards: List[Card] = field(default_factory=list) 

    def __post_init__(self):
        if not self.cards:
            self.cards = [Card(v, s) for v in Value for s in Suits] 
            self.shuffle()

    def __len__(self):
        '''Convenience function.'''
        return len(self.cards)

    def __iter__(self):
        for card in self.cards:
            yield card

    def append(self, card: Card):
        self.cards.append(card)
    
    def shuffle(self):
        random.shuffle(self.cards)
    
    def draw(self) -> Card:
        _card = self.cards.pop()
        print(f"Drew {_card}. {len(self.cards)} cards left.")
        return _card


@total_ordering
class Hand:
    def __init__(self,
                 hole: Tuple[Card, Card],
                 board: List[Card]
                ):
        self.hole = hole
        self.board = board
        self.memo = {}
        self.kicker: int = 0
    
    @property
    def rank(self) -> Rank:
        cards = list(self.hole) + self.board 
        # Get binary representation of this set of cards combination.
        cards_key = sum(1 << card.idx for card in cards)
        
        if cards_key in self.memo:
            return self.memo[cards_key]

        suits = {Suits.CLUBS: [], Suits.HEARTS: [], Suits.SPADES: [], Suits.DIAMONDS: []}
        values = defaultdict(int)

        for card in cards:
            suits[card.suit].append(card.value)
            values[card.value] += 1

        values = sorted(((k.value, v) for k, v in values.items()), key=lambda x: (x[1], x[0]))

        _rank = None
        if self.__is_royal_flush(suits):
            _rank = Rank.ROYAL_FLUSH
        elif self.__is_straight_flush(suits):
            _rank = Rank.STRAIGHT_FLUSH
        elif self.__is_quads(values):
            _rank = Rank.QUADS
        elif self.__is_full_house(values):
            _rank = Rank.FULL_HOUSE
        elif self.__is_flush(suits):
            _rank = Rank.FLUSH
        elif self.__is_straight(values):
            _rank = Rank.STRAIGHT
        elif self.__is_three_of_a_kind(values):
            _rank = Rank.TRIPS
        elif self.__is_two_pair(values):
            _rank = Rank.TWO_PAIR
        elif self.__is_pair(values):
            _rank = Rank.PAIR
        else:
            self.__compute_kicker_as_best_five(5, values)
            _rank = Rank.HIGH_CARD
        self.memo[cards_key] = _rank
        return _rank

    def __is_royal_flush(self, suits: dict[Suits, List[Value]]) -> bool:
        for suit, values in suits.items():
            if Constants.ROYAL_FLUSH_VALUES.issubset(values):
                return True
        return False

    def __is_straight_flush(self, suits: dict[Suits, List[Value]]) -> bool:
        for suit, values in suits.items():
            if len(values) >= 5:
                values = sorted(v.value for v in values)
                # Ace also counts as 1 in a straight flush 
                if values[-1] == 14:
                    values.insert(0, 1)
                for i in range(len(values) - 5, -1, -1): # iterate from back
                    if values[i+4] - values[i] == 4:
                        self.kicker = values[i+4]
                        return True
        return False 

    def __is_quads(self, values: List[Tuple[int, int]]) -> bool:
        if values[-1][1] == 4:
            self.__compute_kicker_as_best_five(2, values)
            return True 
        return False

    def __is_full_house(self, values: List[Tuple[int, int]]) -> bool:
        '''
        For calculation of the kicker:

        The last item of _values corresponds to the highest three-of-a-kind,
        and the second last item corresponds to the highest pair.
        As we first sorted by value count and then by the value of the card.
       
        For example, if we want to compute aces-over-kings is better than kings-over-aces,
        each hand will have the following kicker representation:
        Aces-over-kings: _values[-2:] = [(2, 13), (3, 14)] --> kicker = 1413.
        Kings-over-aces: _values[-2:] = [(2, 14), (3, 13)] --> kicker = 1314.
        Comparing the kickers here, we have Aces-over-kings > Kings-over-aces.
        '''
        if values[-2][1] >= 2 and values[-1][1] >= 3:
            self.__compute_kicker_as_best_five(2, values)
            return True
        return False
        
    def __is_flush(self, suits: dict[Suits, List[Value]]) -> bool:
        for v in suits.values():
            if len(v) >= 5:
                self.kicker = max(v)
                return True
        return False

    def __is_straight(self, values: List[Tuple[int, int]]) -> bool:
        keys = sorted([k for k, v in values if v > 0])
        # Ace also counts as 1 in a straight. 
        if Value.ACE.value == keys[-1]:
            keys.insert(0, 1)

        for i in range(len(keys) - 5, -1, -1):
            if keys[i+4] - keys[i] == 4:
                self.kicker = keys[i+4]
                return True
        return False

    def __is_three_of_a_kind(self, values: List[Tuple[int, int]]) -> bool:
        if values[-1][1] < 3:
            return False

        self.__compute_kicker_as_best_five(3, values)
        return True 
        
    def __is_two_pair(self, values: List[Tuple[int, int]]) -> bool:
        '''
        For the kicker - the 1000s and 100s positions correspond to highest pair
        and the 10s and 1s positions correspond to value of second highest pair.
        This way, we can numerically compute the relative strength between multiple
        2 pair hands by comparing this numeric value.
        
        Example, AcAdJdJs corresponds to the value: 1411.
        The 14 is from the pair of aces and the 11 is from the pair of jacks.
        Suppose we have another hand AcAdQdQs, this has value: 1412.
        By comparing the kicker value, we can see that 1412 > 1411 so AcAdQdQs
        is the winner.
        '''
        if len(values) > 1 and values[-1][1] == 2 and values[-2][1] == 2:
            self.__compute_kicker_as_best_five(3, values)
            return True
        return False

    def __is_pair(self, values: List[Tuple[int, int]]) -> bool:
        if values[-1][1] == 2:
            self.__compute_kicker_as_best_five(4, values)
            return True
        return False

    def __compute_kicker_as_best_five(self, ubound: int, values: List[Tuple[int, int]]):
        _kicker = 0
        for i in range(min(ubound, len(values))):
            _kicker *= 100
            _kicker += values[len(values)-i-1][0]
        self.kicker = _kicker
        
    def __lt__(self, other):
        return self.rank < other.rank or (self.rank == other.rank and self.kicker < other.kicker)

    def __eq__(self, other):
        return self.rank == other.rank and self.kicker == other.kicker


class Game:
    def __init__(self,
                 nplayers: int,
                 hero_pos: int, # 0 being UTG.
                 hands: List[Hand],
                 pot_size: float,
                 board: List[Card],
                 deck: Deck,
                ):
        self.nplayers = nplayers
        self.hero_pos = hero_pos
        self.hands = hands
        self.pot_size = pot_size
        self.board = board
        self.deck = deck

    def outs_one_street(self) -> List[Card]:
        if len(self.board) >= 5:
            return []
        outs = []
        hero = self.hands[self.hero_pos]
        for card in self.deck:
            self.board.append(card)
            if not any(True for (i, villain) in enumerate(self.hands) if hero < villain and i != self.hero_pos):
                outs.append(card)
            self.board.pop()
        return outs

    def compute_odds(self) -> float:
        outs = self.outs_one_street()
        print(f"Outs are {outs}.")
        return len(outs)/len(self.deck)

    def draw_board(self):
        '''
        Randomly draw from the deck.
        This function is not very useful for when we are calculating
        pot odds. But it's a nice function to have to run simulations.
            
        '''
        if len(self.board) == 5:
            return

        # Burn a card.
        self.deck.draw()
        if len(self.board) == 0:
            for i in range(3):
                self.board.append(self.deck.draw())

        elif len(self.board) in [3, 4]:
            self.board.append(self.deck.draw())

    def __iter__(self):
        while len(self.board) < 5:
            yield self.draw_board()
    
    

if __name__ == '__main__':
    deck = Deck()
    hole = (deck.draw(), deck.draw()) #(Card(14, Suits.CLUBS), Card(2, Suits.CLUBS))
    villain_hole = (deck.draw(), deck.draw())
    board = []
    
    print(hole)
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board)
    
    print(sorted([hand, villain_hand]))

    game = Game(nplayers=2,
                hero_pos=0,
                hands=[hand, villain_hand],
                pot_size=5., 
                board=board,
                deck=deck,
                )

    print(game.compute_odds())
    
    # Flop
    game.draw_board()
    print(hand.rank, hand.board)
    print(game.compute_odds())

    # Turn
    game.draw_board()
    print(hand.rank, hand.board)
    print(game.compute_odds())

    # River
    game.draw_board()
    print(hand.rank, hand.board)
