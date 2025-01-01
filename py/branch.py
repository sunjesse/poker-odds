from hand import Deck, Card, Game, Hand, Value, Suits


class BinarySet:
    '''
    Efficient bit set implementation for holding Card types.
    There are 52 cards in total, so we need no more than 52 bits
    to represent the ownership of all cards. Each card has it's own
    consistent index that will be used to determine existence
    inside the set. More space and time efficient than stdlib set
    as we leverage the constraint that we only store up to 52 cards.
    '''
    def __init__(self):
        self.s: int = 0 # i64 as we use at most 52 bits.
        self.length: int = 0

    def add(self, card: Card):
        if not self.contains(card):
            self.s |= 1 << card.idx
            self.length += 1

    def remove(self, card: Card):
        if self.contains(card):
            self.s -= 1 << card.idx
            self.length -= 1

    def contains(self, card: Card) -> bool:
        return (self.s >> card.idx) & 1 == 1

    def __len__(self) -> int:
        return self.length

        
class Brancher:
    def __init__(self,
                 game: Game,
                ):
        self.game: Game = game
        self.hero: Hand = self.game.hands[self.game.hero_pos]
        self.villains: List[Hand] = [hand for i, hand in enumerate(self.game.hands) if i != self.game.hero_pos]
        self.drawn: BinarySet = self.__init_drawn()
        self.memo: dict[int, float] = {}

    def __init_drawn(self) -> int:
        _st = BinarySet()
        for card in self.game.board:
            _st.add(card)

        for hands in self.game.hands:
            for card in hands.hole:
                _st.add(card)
        return _st

    def branch(self) -> float:
        if len(self.game.board) > 5:
            raise Exception("Board has more than 5 cards - invalid.")

        b = self.board_to_bin
        if b in self.memo:
            return self.memo[b]

        if len(self.game.board) == 5:
            val = 0. if any(True for villain in self.villains if self.hero <= villain) else 1.
            if val == 1: # Debugging only!
                print(self.game.board, self.hero.rank)
                for villain in self.villains: print(villain.rank)
            self.memo[b] = val
            return val

        pb = 0
        ncards = len(self.game.deck)
        for card in self.game.deck:
            if self.drawn.contains(card):
                continue
            self.add_to_end_of_board(card)
            pb += self.branch()
            self.remove_from_end_of_board()

        pb /= (ncards - len(self.drawn))
        self.memo[b] = pb 
        return pb 

    def add_to_end_of_board(self, card):
        self.game.board.append(card)
        self.drawn.add(card)

    def remove_from_end_of_board(self):
        self.drawn.remove(self.game.board.pop())

    @property 
    def board_to_bin(self) -> int:
        val = 0
        for card in self.game.board:
            val |= 1 << card.idx 
        return val


if __name__ == "__main__":
    deck = Deck()
    hole = (Card(5, Suits.HEARTS), Card(8, Suits.HEARTS))
    villain_hole = (Card(14, Suits.HEARTS), Card(12, Suits.HEARTS))
    villain2_hole = (deck.draw(), deck.draw())

    board = [
            Card(6, Suits.DIAMONDS),
            Card(9, Suits.HEARTS),
            Card(14, Suits.DIAMONDS),
            ]

    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board) 
    villain2_hand = Hand(villain2_hole, board)

    game = Game(nplayers=2,
            hero_pos=0,
            villain_pos=1,
            hands=[hand, villain_hand, villain2_hand],
            pot_size=5., 
            board=board,
            deck=deck,
            )

    brancher = Brancher(game=game)
    pb = brancher.branch()
    print(f"Hero equity is {pb}.")
