use crate::day4_1;

pub fn run(inp: String) -> String {
    let card_strs: Vec<&str> = inp.split('\n').collect();
    let mut card_count: Vec<usize> = Vec::with_capacity(inp.len());
    for _ in 0..card_strs.len() {
        card_count.push(1);
    }

    for (i, card_str) in card_strs.iter().enumerate() {
        let card = day4_1::parse(card_str);
        let mut indexes_won: usize = 0;
        for ele in card.your_numbers {
            if card.winning_numbers.contains(&ele) {
                indexes_won+=1;
            }
        }
        let current_card_count = card_count.get(i).unwrap_or(&0).clone();
        for i_to_add in i+1..i+indexes_won + 1 {
            if i_to_add < card_count.len() {
                card_count[i_to_add] += current_card_count;
            }
        }
    }

    let mut total_cards = 0;
    for ele in card_count {
        total_cards += ele;
    }
    return format!("{}", total_cards);
}