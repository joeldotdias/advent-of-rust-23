use std::collections::HashMap;

use advent_of_rust_23::input_for_day;

fn main() {
    let sum = input_for_day("02")
        .lines()
        .fold(0, |acc, line| {
            let game_id = line.split(": ").collect::<Vec<&str>>()[0][5..]
                .parse::<u16>()
                .unwrap();
            let picks = line.split(':').collect::<Vec<&str>>()[1]
                .split(';')
                .collect::<Vec<&str>>();

            if is_pick_based(picks) {
                acc + game_id
            } else {
                acc
            }
        });

    println!("{}", sum);
}

fn is_pick_based(picks: Vec<&str>) -> bool {
    let mut flag = false;

    let stupid_lookup = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    picks
        .iter()
        .map(|pick| pick.trim())
        .for_each(|colours| {
            colours
                .split(", ")
                .for_each(|colour| {
                    let parts = colour.split(' ').collect::<Vec<&str>>();
                    let n = parts[0].parse::<u16>().unwrap();
                    // let colour = String::from(parts[1]);
                    let colour = parts[1];

                    if n > *stupid_lookup.get(colour).unwrap() {
                        flag = true;
                    }
                });
        });

    !flag
}

