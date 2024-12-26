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

    def shuffle(self):
        random.shuffle(self.cards)
    
    def draw(self):
        _card = self.cards.pop()
        print(f"Drew {_card}. {len(self.cards)} cards left.")
        return _card


class Hand:
    def __init__(self,
                 hole: Tuple[Card, Card],
                 board: List[Card],
                 deck: Deck):
        self.hole = hole
        self.board = board
        self.deck = deck
        self.log = {}
    
    @property
    def rank(self):
        cards = list(self.hole) + self.board 
        cards_key = tuple(cards)
        if cards_key in self.log:
            return self.log[cards_key]

        suits = {Suits.CLUBS: [], Suits.HEARTS: [], Suits.SPADES: [], Suits.DIAMONDS: []}
        values = defaultdict(int)

        for card in cards:
            suits[card.suit].append(card.value)
            values[card.value] += 1

        max_value = max(values.values())

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
        elif max_value == 3:
            _rank = Rank.TRIPS
        elif self.__is_two_pair(values):
            _rank = Rank.TWO_PAIR
        elif max_value == 2:
            _rank = Rank.PAIR
        else:
            _rank = Rank.HIGH_CARD
        self.log[cards_key] = _rank
        return _rank

    def draw_board(self):
        if len(self.board) == 5:
            return

        # Burn a card.
        self.deck.draw()
        if len(self.board) == 0:
            for i in range(3):
                self.board.append(self.deck.draw())

        elif len(self.board) in [3, 4]:
            self.board.append(self.deck.draw())

    def __is_royal_flush(self, suits):
        royal_flush_values = {Value.TEN, Value.JACK, Value.QUEEN, Value.KING, Value.ACE}
        for suit, values in suits.items():
            if royal_flush_values.issubset(values):
                return True
        return False

    def __is_straight_flush(self, suits):
        for suit, values in suits.items():
            if len(values) >= 5:
                values = sorted(v.value for v in values)
                # Ace also counts as 1 in a straight flush 
                if values[-1] == 14:
                    values.insert(0, 1)
                for i in range(len(values) - 5):
                    if values[i+4] - values[i] == 4: return True
        return False 

    def __is_quads(self, values):
        for v in values.values():
            if v == 4:
                return True 
        return False

    def __is_full_house(self, values):
        _values = sorted(values.values())
        if _values[-2] >= 2 and _values[-1] >= 3:
            return True
        return False
        
    def __is_flush(self, suits):
        for v in suits.values():
            if len(v) >= 5:
                return True
        return False

    def __is_straight(self, values):
        _values = sorted(k.value for k, v in values.items() if v > 0)
        # Ace also counts as 1 in a straight. 
        if Value.ACE in values:
            _values.insert(0, 1)

        for i in range(len(_values) - 4):
            if _values[i+4] - _values[i] == 4:
                return True
        return False

    def __is_two_pair(self, values):
        c = 0
        for v in values.values():
            if v == 2:
                c += 1
        return c >= 2


if __name__ == '__main__':
    deck = Deck()
    hole = (deck.draw(), deck.draw()) #(Card(14, Suits.CLUBS), Card(2, Suits.CLUBS))
    board = []
    print(hole)

    hand = Hand(hole, board, deck)
    print(hand.rank, hand.board)

    # Flop
    hand.draw_board()
    print(hand.rank, hand.board)

    # Turn
    hand.draw_board()
    print(hand.rank, hand.board)

    # River
    hand.draw_board()
    print(hand.rank, hand.board)
    print(len(deck))
