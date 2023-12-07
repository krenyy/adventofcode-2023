fn num(line: &[char], pos: usize) -> u128 {
    let mut start = pos;
    let mut end = pos;

    while start > 0 && line[start - 1].is_numeric() {
        start -= 1;
    }
    while end < line.len() - 1 && line[end + 1].is_numeric() {
        end += 1;
    }

    line[start..=end]
        .into_iter()
        .collect::<String>()
        .parse::<u128>()
        .unwrap()
}

fn main() {
    let lines = std::io::stdin().lines();

    let lines = lines
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] != '*' {
                continue;
            }
            sum += match (
                lines
                    .get(i - 1)
                    .and_then(|x| x.get(j - 1))
                    .map(|x| x.is_numeric()),
                lines
                    .get(i - 1)
                    .and_then(|x| x.get(j))
                    .map(|x| x.is_numeric()),
                lines
                    .get(i - 1)
                    .and_then(|x| x.get(j + 1))
                    .map(|x| x.is_numeric()),
                lines
                    .get(i)
                    .and_then(|x| x.get(j - 1))
                    .map(|x| x.is_numeric()),
                lines
                    .get(i)
                    .and_then(|x| x.get(j + 1))
                    .map(|x| x.is_numeric()),
                lines
                    .get(i + 1)
                    .and_then(|x| x.get(j - 1))
                    .map(|x| x.is_numeric()),
                lines
                    .get(i + 1)
                    .and_then(|x| x.get(j))
                    .map(|x| x.is_numeric()),
                lines
                    .get(i + 1)
                    .and_then(|x| x.get(j + 1))
                    .map(|x| x.is_numeric()),
            ) {
                (Some(true), Some(false), Some(true), _, _, _, _, _) => {
                    num(&lines[i - 1], j - 1) * num(&lines[i - 1], j + 1)
                }
                (_, _, _, _, _, Some(true), Some(false), Some(true)) => {
                    num(&lines[i + 1], j - 1) * num(&lines[i + 1], j + 1)
                }
                (_, _, _, Some(true), Some(true), _, _, _) => {
                    num(&lines[i], j - 1) * num(&lines[i], j + 1)
                }
                (Some(true), _, _, Some(true), _, _, _, _) => {
                    num(&lines[i - 1], j - 1) * num(&lines[i], j - 1)
                }
                (Some(true), _, _, _, Some(true), _, _, _) => {
                    num(&lines[i - 1], j - 1) * num(&lines[i], j + 1)
                }
                (Some(true), _, _, _, _, Some(true), _, _) => {
                    num(&lines[i - 1], j - 1) * num(&lines[i + 1], j - 1)
                }
                (Some(true), _, _, _, _, _, Some(true), _) => {
                    num(&lines[i - 1], j - 1) * num(&lines[i + 1], j)
                }
                (Some(true), _, _, _, _, _, _, Some(true)) => {
                    num(&lines[i - 1], j - 1) * num(&lines[i + 1], j + 1)
                }
                (_, Some(true), _, Some(true), _, _, _, _) => {
                    num(&lines[i - 1], j) * num(&lines[i], j - 1)
                }
                (_, Some(true), _, _, Some(true), _, _, _) => {
                    num(&lines[i - 1], j) * num(&lines[i], j + 1)
                }
                (_, Some(true), _, _, _, Some(true), _, _) => {
                    num(&lines[i - 1], j) * num(&lines[i + 1], j - 1)
                }
                (_, Some(true), _, _, _, _, Some(true), _) => {
                    num(&lines[i - 1], j) * num(&lines[i + 1], j)
                }
                (_, Some(true), _, _, _, _, _, Some(true)) => {
                    num(&lines[i - 1], j) * num(&lines[i + 1], j + 1)
                }
                (_, _, Some(true), Some(true), _, _, _, _) => {
                    num(&lines[i - 1], j + 1) * num(&lines[i], j - 1)
                }
                (_, _, Some(true), _, Some(true), _, _, _) => {
                    num(&lines[i - 1], j + 1) * num(&lines[i], j + 1)
                }
                (_, _, Some(true), _, _, Some(true), _, _) => {
                    num(&lines[i - 1], j + 1) * num(&lines[i + 1], j - 1)
                }
                (_, _, Some(true), _, _, _, Some(true), _) => {
                    num(&lines[i - 1], j + 1) * num(&lines[i + 1], j)
                }
                (_, _, Some(true), _, _, _, _, Some(true)) => {
                    num(&lines[i - 1], j + 1) * num(&lines[i + 1], j + 1)
                }
                (_, _, _, Some(true), _, Some(true), _, _) => {
                    num(&lines[i], j - 1) * num(&lines[i + 1], j - 1)
                }
                (_, _, _, Some(true), _, _, Some(true), _) => {
                    num(&lines[i], j - 1) * num(&lines[i + 1], j)
                }
                (_, _, _, Some(true), _, _, _, Some(true)) => {
                    num(&lines[i], j - 1) * num(&lines[i + 1], j + 1)
                }
                (_, _, _, _, Some(true), Some(true), _, _) => {
                    num(&lines[i], j + 1) * num(&lines[i + 1], j - 1)
                }
                (_, _, _, _, Some(true), _, Some(true), _) => {
                    num(&lines[i], j + 1) * num(&lines[i + 1], j)
                }
                (_, _, _, _, Some(true), _, _, Some(true)) => {
                    num(&lines[i], j + 1) * num(&lines[i + 1], j + 1)
                }
                _ => 0,
            }
        }
    }

    println!("{sum}");
}
