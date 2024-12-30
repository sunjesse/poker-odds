from hand import Deck, Card, Game, Hand, Value, Suits


class Brancher:
    def __init__(self,
                 game: Game,
                ):
        self.game = game
        self.hero = self.game.hands[self.game.hero_pos]
        self.villain = self.game.hands[self.game.villain_pos]
        self.drawn = set(self.game.board)

    def branch(self, layer=0) -> float:
        if len(self.game.board) > 5:
            raise Exception("Board has more than 5 cards - invalid.")

        if len(self.game.board) == 5:
            return 1. if self.hero > self.villain else 0.
        pb = 0
        ncards = len(self.game.deck)
        for card in self.game.deck:
            if card in self.drawn:
                continue
            self.add_to_end_of_board(card)
            pb += self.branch(layer+1)
            self.remove_from_end_of_board()
        return pb / (ncards - len(self.drawn))

    def add_to_end_of_board(self, card):
        self.game.board.append(card)
        self.drawn.add(card)

    def remove_from_end_of_board(self):
        self.drawn.remove(self.game.board.pop())


if __name__ == "__main__":
    deck = Deck()
    hole = (Card(14, Suits.CLUBS), Card(14, Suits.DIAMONDS))
    villain_hole = (Card(13, Suits.CLUBS), Card(13, Suits.DIAMONDS))

    board = [Card(11, Suits.SPADES), Card(11, Suits.DIAMONDS), Card(11, Suits.CLUBS)]
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
