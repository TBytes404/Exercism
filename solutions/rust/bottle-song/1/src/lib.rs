pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(n: u32) -> String {
    let curr = bottle(n as usize);
    let next = bottle((n - 1) as usize);
    let cap_curr = curr[..1].to_uppercase() + &curr[1..];
    format!(
        "{cap_curr} hanging on the wall,
{cap_curr} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {next} hanging on the wall."
    )
}

fn bottle(n: usize) -> String {
    DIGITS[n].to_string() + " green bottle" + if n == 1 { "" } else { "s" }
}

const DIGITS: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];
