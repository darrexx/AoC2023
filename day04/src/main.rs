fn main() {
    let input = include_str!("../input.txt");
    let cards = input
        .lines()
        .map(|x| x.split([':', '|']))
        .map(|mut x| Card {
            winning_numbers: x
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|y| y.parse().unwrap())
                .collect(),
            numbers: x
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|y| y.parse().unwrap())
                .collect(),
        })
        .collect::<Vec<_>>();

    let part1 = part1(cards.clone());
    let part2 = part2(cards);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

#[derive(Clone)]
struct Card {
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

fn part1(cards: Vec<Card>) -> usize {
    let mut sum = 0;
    for card in cards {
        let amount_of_winning_numbers = get_amount_of_winning_numbers(&card) as u32;

        if amount_of_winning_numbers == 0 {
            continue;
        }

        sum = sum + 2usize.pow(amount_of_winning_numbers - 1);
    }

    sum
}

fn part2(cards: Vec<Card>) -> usize {
    let mut card_amounts = cards.iter().map(|_| 1).collect::<Vec<_>>();
    for (i, card) in cards.iter().enumerate() {
        let winners = get_amount_of_winning_numbers(card);
        for _ in 0..card_amounts[i] {
            for next_cards in i + 1..i + 1 + winners {
                card_amounts[next_cards] = card_amounts[next_cards] + 1;
            }
        }
    }
    card_amounts.iter().sum()
}

fn get_amount_of_winning_numbers(card: &Card) -> usize {
    card.numbers
        .iter()
        .filter(|x| card.winning_numbers.contains(x))
        .count()
}
