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

        let tuples = game
            .sets
            .into_iter()
            .map(|x| {
                (
                    *x.get("red").unwrap_or(&0),
                    *x.get("green").unwrap_or(&0),
                    *x.get("blue").unwrap_or(&0),
                )
            })
            .collect::<Vec<_>>();

        let r_max = tuples.iter().map(|x| x.0).max().unwrap();
        let g_max = tuples.iter().map(|x| x.1).max().unwrap();
        let b_max = tuples.iter().map(|x| x.2).max().unwrap();

        sum += r_max * g_max * b_max;
    }

    println!("{sum}");
}
