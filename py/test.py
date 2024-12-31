from hand import Deck, Hand, Card, Suits


def test_full_house_over_full_house():    
    deck = Deck()
    hole = (Card(14, Suits.CLUBS), Card(14, Suits.DIAMONDS))
    villain_hole = (Card(13, Suits.CLUBS), Card(13, Suits.DIAMONDS))

    board = [Card(11, Suits.SPADES), Card(11, Suits.DIAMONDS), Card(11, Suits.CLUBS)]
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board)

    print("====== FULL HOUSE OVER FULL HOUSE =====")
    print(f"Your rank: {hand.rank}\nVillain rank: {villain_hand.rank}")
    print(f"Your kicker: {hand.kicker} \nVillain kicker: {villain_hand.kicker}\nhand > villain_hand: {hand > villain_hand}")
    
    assert (hand > villain_hand) == True 


def test_two_pair_over_two_pair():    
    deck = Deck()
    hole = (Card(14, Suits.CLUBS), Card(14, Suits.DIAMONDS))
    villain_hole = (Card(13, Suits.CLUBS), Card(13, Suits.DIAMONDS))

    board = [Card(10, Suits.SPADES), Card(11, Suits.DIAMONDS), Card(11, Suits.CLUBS)]
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board)

    print("====== TWO PAIR OVER TWO PAIR =====")
    print(f"Your rank: {hand.rank}\nVillain rank: {villain_hand.rank}")
    print(f"Your kicker: {hand.kicker} \nVillain kicker: {villain_hand.kicker}\nhand > villain_hand: {hand > villain_hand}")
    
    assert (hand > villain_hand) == True 


def test_same_three_of_a_kind_diff_kicker():    
    deck = Deck()
    hole = (Card(14, Suits.CLUBS), Card(13, Suits.DIAMONDS))
    villain_hole = (Card(14, Suits.CLUBS), Card(12, Suits.DIAMONDS))

    board = [Card(14, Suits.SPADES), Card(14, Suits.HEARTS), Card(11, Suits.CLUBS)]
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board)

    print("====== SAME THREE OF A KIND, DIFFERENT KICKER =====")
    print(f"Your rank: {hand.rank}\nVillain rank: {villain_hand.rank}")
    print(f"Your kicker: {hand.kicker} \nVillain kicker: {villain_hand.kicker}\nhand > villain_hand: {hand > villain_hand}")
    
    assert (hand > villain_hand) == True 


def test_pair_with_kicker():    
    deck = Deck()
    hole = (Card(14, Suits.CLUBS), Card(13, Suits.DIAMONDS))
    villain_hole = (Card(14, Suits.SPADES), Card(3, Suits.DIAMONDS))

    board = [Card(10, Suits.SPADES), Card(14, Suits.DIAMONDS), Card(11, Suits.CLUBS)]
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board)

    print("====== TWO PAIR OVER TWO PAIR =====")
    print(f"Your rank: {hand.rank}\nVillain rank: {villain_hand.rank}")
    print(f"Your kicker: {hand.kicker} \nVillain kicker: {villain_hand.kicker}\nhand > villain_hand: {hand > villain_hand}")
    
    assert (hand > villain_hand) == True 


if __name__ == "__main__":
    test_full_house_over_full_house()
    test_two_pair_over_two_pair()
    test_same_three_of_a_kind_diff_kicker()
    test_pair_with_kicker()
