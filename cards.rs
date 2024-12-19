

enum CardType {
    Hearts, Diamonds, Clubs, Spades
}

enum Value {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

struct Card {
    type: CardType,
    value: Value
}

impl Card {
    fn value(&self) -> u8 {
        match self.value {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten | Value::Jack | Value::Queen | Value::King => 10,
            Value::Ace => 11,
        }
    }
}

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();
        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for value in &[Value::Two, Value::Three, Value::Four, Value::Five, Value::Six,
                          Value::Seven, Value::Eight, Value::Nine, Value::Ten, Value::Jack,
                          Value::Queen, Value::King, Value::Ace] {
                cards.push(Card { suit: *suit, value: *value });
            }
        }
        let mut randInt = rand::thread_rng();
        cards.shuffle(&mut randInt);
        Deck {cards}
    }

    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

struct Player {
    hand: Vec<Card>
}

impl Player {
    fn score(&self) -> u8 {
        let mut score = self.hand.iter().map(|c| c.value()).sum::<u8>();
        let ace_count = self.hand.iter().filter(|card| matches!(card.value, Value::Ace)).count();

        for _ in 0..ace_count {
            if score > 21 {
                score -= 10;
            }
        }
        score
    }
}