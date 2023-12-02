
use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


fn is_one(text: &str) -> bool {
    return text.chars().collect::<String>().starts_with("one");
}

fn is_two(text: &str) -> bool {
    text.chars().collect::<String>().starts_with("two")
}

fn is_three(text: &str) -> bool {
    text.chars().collect::<String>().starts_with("three")
}

fn is_four(text: &str) -> bool {
    text.chars().collect::<String>().starts_with("four")
}

fn is_five(text: &str) -> bool {
    text.chars().collect::<String>().starts_with("five")
}

fn is_six(text: &str) -> bool {
    text.chars().collect::<String>().starts_with("six")
}

fn is_seven(text: &str) -> bool {
    text.chars().collect::<String>().starts_with("seven")
}

fn is_eight(text: &str) -> bool {
    text.chars().collect::<String>().starts_with("eight")
}

fn is_nine(text: &str) -> bool {
    text.chars().collect::<String>().starts_with("nine")
}

fn get_value_if_exists(text: &str) -> Option<u32> {
    if is_one(text) { return Some(1); }
    if is_two(text) { return Some(2); }
    if is_three(text) { return Some(3); }
    if is_four(text) { return Some(4); }
    if is_five(text) { return Some(5); }
    if is_six(text) { return Some(6); }
    if is_seven(text) { return Some(7); }
    if is_eight(text) { return Some(8); }
    if is_nine(text) { return Some(9); }

    None
}


fn two_digits(text: &str) -> u32 {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for i in 0..text.len() {
        let remaining_text = &text[i..text.len()];
        let char_at_0 = remaining_text.chars().nth(0).unwrap();

        if char_at_0.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(char_at_0.to_digit(10).unwrap());
            }
            last_digit = Some(char_at_0.to_digit(10).unwrap());
            continue;
        }

        let value_string = get_value_if_exists(remaining_text);
        if value_string.is_some() {
            if first_digit.is_none() {
                first_digit = value_string;
            }
            last_digit = value_string;
        }
    }

    (first_digit.unwrap() * 10) + last_digit.unwrap()

}

pub(crate) fn day_one() {
    let lines = read_lines("src/day_one/adventFile.txt");

    let total: u32 = lines.iter().map(|line| two_digits(line)).sum();

    print!("Total: {}", total)
}