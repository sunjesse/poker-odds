from hand import Deck, Card, Game, Hand, Value, Suits


class Brancher:
    def __init__(self,
                 game: Game,
                ):
        self.game = game
        self.hero = self.game.hands[self.game.hero_pos]
        self.villains = [hand for i, hand in enumerate(self.game.hands) if i != self.game.hero_pos]
        self.drawn_ct = 0
        self.drawn = self.__init_drawn()
        self.memo = {}
        print(self.villains)

    def __init_drawn(self) -> int:
        _st = 0
        for card in self.game.board:
            _st |= 1 << card.idx
            self.drawn_ct += 1

        for hands in self.game.hands:
            for card in hands.hole:
                _st |= 1 << card.idx
                self.drawn_ct += 1
        return _st

    def branch(self) -> float:
        if len(self.game.board) > 5:
            raise Exception("Board has more than 5 cards - invalid.")

        b = self.board_to_bin
        if b in self.memo:
            return self.memo[b]

        if len(self.game.board) == 5:
            val = 0. if any(self.hero < villain for villain in self.villains) else 1.
            if val == 1:
                print(self.game.board, self.hero.rank)
                for villain in self.villains: print(villain.rank)
            self.memo[b] = val
            return val

        pb = 0
        ncards = len(self.game.deck)
        for card in self.game.deck:
            if (self.drawn >> card.idx) & 1:
                continue
            self.add_to_end_of_board(card)
            pb += self.branch()
            self.remove_from_end_of_board()

        pb /= (ncards - self.drawn_ct)
        self.memo[b] = pb 
        return pb 

    def add_to_end_of_board(self, card):
        self.game.board.append(card)
        self.drawn |= 1 << card.idx
        self.drawn_ct += 1

    def remove_from_end_of_board(self):
        self.drawn -= 1 << self.game.board.pop().idx
        self.drawn_ct -= 1

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
