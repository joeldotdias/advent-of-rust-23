use advent_of_rust_23::input_for_day;

struct SymCoord {
    x: usize,
    y: usize
}

struct NumCoord {
    n: u32,
    x1: usize,
    x2: usize
}

fn main() {
    let mut sum = 0;
    let mut num = String::from("");

    let mut num_lines: Vec<Vec<NumCoord>> = Vec::new();
    let mut sym_coords: Vec<SymCoord> = Vec::new();


    for (y, line) in input_for_day("03")
        .lines()
        .enumerate() {
            let mut nums_in_row: Vec<NumCoord> = Vec::new();

            for (x, ch) in line.chars().enumerate() {
                    if ch.is_ascii_digit() {
                        num.push(ch)
                    } else {
                        if ch != '.' {
                            sym_coords.push(SymCoord{ x, y });
                        }

                        if !num.is_empty() {
                            let realx = if x == 0 { line.len() } else { x };
                            nums_in_row.push(NumCoord{n: num.parse::<u32>().unwrap(), x1: realx - num.len(), x2: realx - 1});
                            num.clear();
                        }
                    }
                }

        num_lines.push(nums_in_row);
    }

    sym_coords.iter().for_each(|sym| {
        let sx = sym.x;
        let sy = sym.y;

        for num_line in num_lines.iter().take(sy + 2).skip(sy - 1) {
            num_line.iter().for_each(|num| {
                let nx1 = num.x1;
                let nx2 = num.x2;
                if nx1 == sx || nx1 == sx - 1 || nx1 == sx + 1 || nx2 == sx || nx2 == sx - 1 || nx2 == sx + 1 {
                    sum += num.n;
                }
            })
        }
    });

    println!("{}", sum);
}
