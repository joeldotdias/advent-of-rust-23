use std::collections::HashMap;

use advent_of_rust_23::input_for_day;

fn main() {
    let sum = input_for_day("02")
        .lines()
        .fold(0, |acc, game| {
            let mut min_balls = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0)
            ]);

            game.split(": ").collect::<Vec<&str>>()[1]
                .split("; ")
                .for_each(|pick| {
                    let draws = pick.split(", ");

                    draws.for_each(|draw| {
                        let parts = draw.split(' ').collect::<Vec<&str>>();
                        let count = parts[0].parse::<u32>().unwrap();
                        let colour = parts[1];

                        if *min_balls.get(colour).unwrap() < count {
                            min_balls.insert(colour, count);
                        }
                    })
                });

            acc + min_balls.get("red").unwrap() * min_balls.get("blue").unwrap() * min_balls.get("green").unwrap()
        });

    println!("{}", sum);
}
