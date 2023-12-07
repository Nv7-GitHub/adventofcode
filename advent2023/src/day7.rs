use std::cmp::Ordering;

pub fn parse_card(card: char) -> usize {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        //'J' => 11, // PART 1
        'J' => 1, // PART 2
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card: {}", card),
    }
}

#[derive(Debug)]
pub struct Card {
    original: [usize; 5],
    value: [usize; 15], // where index is card val and number = # of cards
    bid: usize,
}

impl Card {
    pub fn score(&self) -> usize {
        if self.value[0] == 5 {
            return 7;
        }
        if self.value[0] == 4 {
            return 6;
        }
        if self.value[0] == 3 && self.value[1] == 2 {
            return 5;
        }
        if self.value[0] == 3 {
            return 4;
        }
        if self.value[0] == 2 && self.value[1] == 2 {
            return 3;
        }
        if self.value[0] == 2 {
            return 2;
        }
        1
    }
}

pub fn compare_cards(a: &Card, b: &Card) -> Ordering {
    // Calculate score for a
    if a.score() < b.score() {
        return Ordering::Less;
    } else if a.score() > b.score() {
        return Ordering::Greater;
    }

    // Compare card nums
    for i in 0..5 {
        if a.original[i] < b.original[i] {
            return Ordering::Less;
        } else if a.original[i] > b.original[i] {
            return Ordering::Greater;
        }
    }

    // Equal
    return Ordering::Equal;
}

pub fn day7() {
    // Parse
    let inp: Vec<&str> = include_str!("day7.txt").split("\n").collect();
    let mut cards = Vec::new();
    for line in inp {
        let parts: Vec<&str> = line.split(" ").collect();
        let mut value = [0; 15];
        for ch in parts[0].chars() {
            value[parse_card(ch)] += 1;
        }

        // PART 2
        let cnt_joker = value[1];
        value[1] = 0;
        // PART 2

        value.sort();
        value.reverse();

        // PART 2
        value[0] += cnt_joker;
        // PART 2

        let bid = parts[1].parse::<usize>().unwrap();
        let origvals: Vec<usize> = parts[0].chars().map(|x| parse_card(x)).collect();
        cards.push(Card {
            original: origvals.try_into().unwrap(),
            value,
            bid,
        });
    }

    // Solve
    cards.sort_by(|a, b| compare_cards(a, b));
    let mut res = 0;
    for i in 0..cards.len() {
        res += cards[i].bid * (i + 1);
    }
    println!("Result: {}", res);
}
