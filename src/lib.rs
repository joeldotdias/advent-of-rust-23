use std::fs;

pub fn input_for_day(day: &str) -> String {
    let today = format!("input/day{}.txt", day);

    fs::read_to_string(today)
        .unwrap()
}
