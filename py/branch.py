from hand import Deck, Card, Game, Hand, Value, Suits


class Brancher:
    def __init__(self,
                 game: Game,
                ):
        self.game = game
        self.hero = self.game.hands[self.game.hero_pos]
        self.villain = self.game.hands[self.game.villain_pos]
        self.drawn = self.__init_drawn()
        self.memo = {}

    def __init_drawn(self):
        _st = set()
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
            val = 1. if self.hero > self.villain else 0.
            if val == 1:
                print(self.game.board, self.hero > self.villain, self.hero.rank, self.villain.rank)
            self.memo[b] = val
            return val

        pb = 0
        ncards = len(self.game.deck)
        for card in self.game.deck:
            if card in self.drawn:
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
    def board_to_bin(self):
        val = 0
        for card in self.game.board:
            val |= 1 << card.idx 
        return val


if __name__ == "__main__":
    deck = Deck()
    hole = (Card(5, Suits.HEARTS), Card(8, Suits.HEARTS)) #(Card(14, Suits.CLUBS), Card(14, Suits.DIAMONDS))
    villain_hole = (Card(14, Suits.HEARTS), Card(12, Suits.HEARTS)) #(Card(13, Suits.CLUBS), Card(13, Suits.DIAMONDS))

    board = [ #[Card(11, Suits.SPADES), Card(11, Suits.DIAMONDS), Card(11, Suits.CLUBS)]
            Card(6, Suits.DIAMONDS),
            Card(9, Suits.HEARTS),
            Card(14, Suits.DIAMONDS),
            #Card(4, Suits.CLUBS)
            ]
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board) 

    game = Game(nplayers=2,
            hero_pos=0,
            villain_pos=1,
            hands=[hand, villain_hand],
            pot_size=5., 
            board=board,
            deck=deck,
            )

    brancher = Brancher(game=game)
    pb = brancher.branch()
    print(f"Hero equity is {pb}.")
