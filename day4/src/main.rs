use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
struct Card {
    id: u128,
    winning_numbers: HashSet<u128>,
    numbers: Vec<u128>,
}

fn main() {
    let mut lines = std::io::stdin().lines();

    let mut cards = vec![];
    let mut id = 1;
    while let Some(Ok(line)) = lines.next() {
        let v = line.split(" | ").collect::<Vec<_>>();
        let (first, second) = (v[0], v[1]);

        let first = first.split(':').skip(1).collect::<String>();
        let winning_numbers = first
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<HashSet<_>>();

        let numbers = second
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<_>>();

        cards.push(Card {
            id,
            winning_numbers,
            numbers,
        });
        id += 1;
    }

    let mut card_queue = VecDeque::from(cards.iter().collect::<Vec<_>>());

    let mut card_count = 0;
    while !card_queue.is_empty() {
        let card = card_queue.pop_front().unwrap();
        let x = card
            .numbers
            .iter()
            .map(|n| card.winning_numbers.contains(&n))
            .filter(|a| *a)
            .count();
        for i in card.id..(card.id + x as u128) {
            card_queue.push_back(&cards[i as usize]);
        }
        card_count += 1;
    }

    println!("{card_count}");
}
