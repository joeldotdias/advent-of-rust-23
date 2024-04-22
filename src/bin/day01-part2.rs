use advent_of_rust_23::input_for_day;

fn main() {
    let sum = input_for_day("01")
        .lines()
        .fold(0, |acc, line| {
            let line = line
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");

            acc + num_from_line(&line)
        });

    println!("{}", sum);
}

fn num_from_line(line: &str) -> u32 {
    let mut left_most = 0;
    let mut right_most = 0;

    line.chars().for_each(|ch| {
        if ch.is_ascii_digit() {
            let num = ch.to_digit(10).unwrap();

            if left_most == 0 {
                left_most = num;
                right_most = num;
            } else {
                right_most = num;
            }
        };
    });

    left_most * 10 + right_most
}
