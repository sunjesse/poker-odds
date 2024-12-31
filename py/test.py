from hand import Deck, Hand, Card, Suits


def test_full_house_over_full_house():    
    deck = Deck()
    hole = (Card(14, Suits.CLUBS), Card(14, Suits.DIAMONDS))
    villain_hole = (Card(13, Suits.CLUBS), Card(13, Suits.DIAMONDS))

    board = [Card(11, Suits.SPADES), Card(11, Suits.DIAMONDS), Card(11, Suits.CLUBS)]
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board)

    print("\n====== FULL HOUSE OVER FULL HOUSE =====")
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

    print("\n====== TWO PAIR OVER TWO PAIR =====")
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

    print("\n====== SAME THREE OF A KIND, DIFFERENT KICKER =====")
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

    print("\n====== PAIR WITH KICKER =====")
    print(f"Your rank: {hand.rank}\nVillain rank: {villain_hand.rank}")
    print(f"Your kicker: {hand.kicker} \nVillain kicker: {villain_hand.kicker}\nhand > villain_hand: {hand > villain_hand}")
    
    assert (hand > villain_hand) == True 


def test_high_card():    
    deck = Deck()
    hole = (Card(8, Suits.CLUBS), Card(11, Suits.DIAMONDS))
    villain_hole = (Card(7, Suits.SPADES), Card(11, Suits.DIAMONDS))

    board = [Card(13, Suits.SPADES), Card(12, Suits.DIAMONDS), Card(14, Suits.CLUBS)]
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board)

    print("\n====== HIGH CARD TEST KICKER =====")
    print(f"Your rank: {hand.rank}\nVillain rank: {villain_hand.rank}")
    print(f"Your kicker: {hand.kicker} \nVillain kicker: {villain_hand.kicker}\nhand > villain_hand: {hand > villain_hand}")
    
    assert (hand > villain_hand) == True 


def test_multiway_showdown():    
    deck = Deck()
    hole = (Card(8, Suits.CLUBS), Card(11, Suits.DIAMONDS))
    villain_hole = (Card(7, Suits.SPADES), Card(11, Suits.DIAMONDS))
    villain2_hole = (Card(6, Suits.DIAMONDS), Card(5, Suits.DIAMONDS))

    board = [
            Card(8, Suits.SPADES),
            Card(8, Suits.DIAMONDS),
            Card(11, Suits.CLUBS),
            Card(6, Suits.CLUBS),
            Card(6, Suits.SPADES)
            ]
    hand = Hand(hole, board)
    villain_hand = Hand(villain_hole, board)
    villain2_hand = Hand(villain2_hole, board)

    print("\n====== HIGH CARD TEST KICKER =====")
    print(f"Your rank: {hand.rank}\nVillain rank: {villain_hand.rank}\nVillain 2 rank: {villain2_hand.rank}")
    print(f"Your kicker: {hand.kicker} \nVillain kicker: {villain_hand.kicker}\nVillain 2 kicker: {villain2_hand.kicker}")
    
    assert (hand > villain_hand) == True 
    assert (hand > villain2_hand) == True
    assert (villain2_hand > villain_hand) == True


if __name__ == "__main__":
    test_full_house_over_full_house()
    test_two_pair_over_two_pair()
    test_same_three_of_a_kind_diff_kicker()
    test_pair_with_kicker()
    test_high_card()
    test_multiway_showdown()
