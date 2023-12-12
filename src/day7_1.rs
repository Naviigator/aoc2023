pub fn run(inp: String) -> String {
    let mut hands: Vec<Hand> = Vec::new();
    for ele in inp.split('\n') {
        hands.push(parse_hand(ele));
    }
    hands.sort();
    let mut score = 0;
    for ele in hands.iter().enumerate() {
        score += (ele.0+1) * ele.1.bet;
    }

    return format!("{}", score);
}

pub fn parse_hand(inp: &str) -> Hand {
    let split: Vec<&str> = inp.split(' ').collect();
    let card_string = split.get(0).unwrap();
    let cards = get_cards(card_string);

    Hand{
        hand_type: get_hand_type(&cards),
        cards: cards,
        bet: split.get(1).unwrap().parse().unwrap(),
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub struct Hand {
    pub hand_type: HandType,
    pub cards: Vec<Card>,
    pub bet: usize,
}

pub fn get_hand_type(cards: &Vec<Card>) -> HandType {
    let mut unique_cards: Vec<Card> = cards.to_vec();
    unique_cards.sort();
    unique_cards.dedup();
    if unique_cards.len() == 1 {
        return HandType::FiveOfAKind;
    }
    if unique_cards.len() == 5 {
        return HandType::HighCard;
    }
    if unique_cards.len() == 4 {
        return HandType::Pair;
    }
    if unique_cards.len() == 2 {
        for unique_card in &unique_cards {
            let mut count = 0;
            for c in cards {
                if c == unique_card {
                    count+=1;
                }
            }
            if count == 4 {
                return HandType::FourOfAKind;
            }
        }
        return HandType::FullHouse;
    }
    //3 unique chars remain - suspense rises!
    for unique_card in &unique_cards {
        let mut count = 0;
        for c in cards {
            if c == unique_card {
                count+=1;
            }
        }
        if count == 3 {
            return HandType::ThreeOfAKind;
        }
    }
    return HandType::TwoPair;
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn get_cards(inp: &str) -> Vec<Card> {
    let mut result = Vec::new();
    for ele in inp.chars() {
        result.push(get_card(ele));
    }
    result
}

pub fn get_card(inp: char) -> Card {
    match inp {
        '2'=>Card::Two,
        '3'=>Card::Three,
        '4'=>Card::Four,
        '5'=>Card::Five,
        '6'=>Card::Six,
        '7'=>Card::Seven,
        '8'=>Card::Eight,
        '9'=>Card::Nine,
        'T'=>Card::Ten,
        'J'=>Card::Jack,
        'Q'=>Card::Queen,
        'K'=>Card::King,
        'A'=>Card::Ace,
        _=>panic!("bad card")
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,//elf cards, they're special
    Jack,
    Queen,
    King,
    Ace,
}