use std::collections::HashSet;

fn main() {
    let mut lines = std::io::stdin().lines();

    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        let v = line.split(" | ").collect::<Vec<_>>();
        let (first, second) = (v[0], v[1]);

        let first = first.split(':').skip(1).collect::<String>();
        let winning_numbers = first
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<HashSet<_>>();

        let hand = second
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u128>().unwrap())
            .collect::<Vec<_>>();

        let mut card_value = 0;
        for num in hand {
            if winning_numbers.contains(&num) {
                if card_value == 0 {
                    card_value = 1;
                } else {
                    card_value *= 2;
                }
            }
        }
        sum += card_value;
    }

    println!("{sum}");
}
