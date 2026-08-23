pub fn verse(n: u32) -> String {
    let current = bottle_count(n);
    let next = bottle_count(n - 1);

    let current_capitalized = capitalize(current);

    format!(
        "{current_capitalized} green {} hanging on the wall,\n\
         {current_capitalized} green {} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {next} green {} hanging on the wall.",
        bottle_word(n),
        bottle_word(n),
        bottle_word(n - 1),
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn bottle_count(n: u32) -> &'static str {
    match n {
        0 => "no",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => unreachable!(),
    }
}

fn bottle_word(n: u32) -> &'static str {
    if n == 1 {
        "bottle"
    } else {
        "bottles"
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}