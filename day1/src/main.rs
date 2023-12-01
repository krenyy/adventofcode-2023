fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lines();

    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        let i = line.chars().find(|c| c.is_numeric());
        let j = line.chars().rev().find(|c| c.is_numeric());
        let mut s = String::new();
        if i.is_none() || j.is_none() {
            continue;
        }
        s.push(i.unwrap());
        s.push(j.unwrap());
        sum += s.parse::<u128>().unwrap();
    }

    println!("{sum}")
}
