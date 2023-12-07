fn asdf(start: Option<&(usize, usize)>, end: Option<&(usize, usize)>, lines: &[Vec<char>]) -> u128 {
    if start.is_none() || end.is_none() {
        return 0;
    }
    let start = start.unwrap();
    let end = end.unwrap();
    let k_range = (end.0 as i64 - 1)..=(end.0 as i64 + 1);
    let l_range = (end.1 as i64 - (end.1 - start.1) as i64 - 1)..=(end.1 as i64 + 1);

    let mut is_part = false;
    'a: for k in k_range.clone() {
        for l in l_range.clone() {
            if (k != *k_range.start()
                && k != *k_range.end()
                && l != *l_range.start()
                && l != *l_range.end())
                || k < 0
                || k >= lines.len() as i64
                || l < 0
                || l >= lines[k as usize].len() as i64
            {
                continue;
            }
            if lines[k as usize][l as usize] != '.' {
                is_part = true;
                break 'a;
            }
        }
    }

    if is_part {
        let num = lines[start.0][start.1..=end.1]
            .to_vec()
            .into_iter()
            .collect::<String>()
            .parse::<u128>()
            .unwrap();
        return num;
    }

    return 0;
}

fn main() {
    let lines = std::io::stdin().lines();

    let lines = lines
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..lines.len() {
        let mut start = None;
        let mut end = None;
        for j in 0..lines[i].len() {
            if lines[i][j].is_numeric() {
                if start.is_none() {
                    start = Some((i, j));
                }
                end = Some((i, j));
                continue;
            }
            sum += asdf(start.as_ref(), end.as_ref(), &lines);
            start = None;
            end = None;
        }
        sum += asdf(start.as_ref(), end.as_ref(), &lines);
    }

    println!("{sum}");
}
