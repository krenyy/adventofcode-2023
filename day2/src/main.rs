use std::{collections::HashMap, io};

#[derive(Debug)]
struct Game<'a> {
    id: usize,
    sets: Vec<HashMap<&'a str, usize>>,
}

fn main() {
    let mut lines = io::stdin().lines();

    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        let id = line
            .split(':')
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let x = line
            .split(' ')
            .skip(2)
            .fold(String::new(), |a, b| a + b + " ");
        let y = x
            .split(';')
            .map(|s| {
                s.split(',')
                    .map(|s| s.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>())
                    .map(|x| (x[1], x[0].parse::<usize>().unwrap()))
                    .collect::<HashMap<_, _>>()
            })
            .collect::<Vec<_>>();

        let game = Game {
            id,
            sets: y.clone(),
        };

        let mut valid = true;
        for game_set in game.sets {
            if *game_set.get("red").unwrap_or(&0) > 12
                || *game_set.get("green").unwrap_or(&0) > 13
                || *game_set.get("blue").unwrap_or(&0) > 14
            {
                valid = false;
                break;
            }
        }
        if valid {
            sum += game.id;
        }
    }

    println!("{sum}");
}
