use regex::Regex;

fn word_to_num(w: &str) -> &str {
    match w {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => unreachable!(),
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lines();

    let regexes = [
        Regex::new(r"1").unwrap(),
        Regex::new(r"2").unwrap(),
        Regex::new(r"3").unwrap(),
        Regex::new(r"4").unwrap(),
        Regex::new(r"5").unwrap(),
        Regex::new(r"6").unwrap(),
        Regex::new(r"7").unwrap(),
        Regex::new(r"8").unwrap(),
        Regex::new(r"9").unwrap(),
        Regex::new("one").unwrap(),
        Regex::new("two").unwrap(),
        Regex::new("three").unwrap(),
        Regex::new("four").unwrap(),
        Regex::new("five").unwrap(),
        Regex::new("six").unwrap(),
        Regex::new("seven").unwrap(),
        Regex::new("eight").unwrap(),
        Regex::new("nine").unwrap(),
    ];

    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        let mut matches = regexes
            .iter()
            .map(|rgx| rgx.find_iter(&line))
            .flat_map(|x| x)
            .collect::<Vec<_>>();
        matches.sort_by_key(|x| x.start());
        let first = matches.iter().next().unwrap().as_str();
        let last = matches.iter().last().unwrap().as_str();
        let first = if first.len() > 1 {
            word_to_num(first)
        } else {
            first
        };
        let last = if last.len() > 1 {
            word_to_num(last)
        } else {
            last
        };
        let mut s = String::new();
        s.push_str(first);
        s.push_str(last);
        sum += s.parse::<u128>().unwrap();
    }

    println!("{sum}")
}
