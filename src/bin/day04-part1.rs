use advent_of_rust_23::input_for_day;

struct Card {
    win_nums: Vec<u32>,
    my_nums: Vec<u32>
}

fn nums_from_section(section: &str) -> Vec<u32> {
    section.split(' ')
        .filter_map(|w|
            (w.parse::<u32>().is_ok())
                .then(|| w.parse::<u32>().unwrap())
        ).collect::<Vec<u32>>()
}

fn main() {
    let mut cards: Vec<Card> = Vec::new();

    input_for_day("04")
        .lines()
        .for_each(|line| {
            let sections: Vec<&str> = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect();

            cards.push(Card{
                win_nums: nums_from_section(sections[0]),
                my_nums: nums_from_section(sections[1]),
            });
        });

    let score = cards.iter().
        fold(0, |acc, card| {
        let mut matches = 0;

        card.win_nums.iter().for_each(|w| {
            if card.my_nums.contains(w) {
                matches += 1;
            }
        });

        if matches != 0 {
            acc + u32::pow(2, matches - 1)
        } else {
            acc
        }
    });

    println!("{}", score);
}

