use advent_of_rust_23::input_for_day;

struct Guide {
    dest: u64,
    src: u64,
    range: u64
}

fn main() {
    let mut seeds = Vec::<u64>::new();
    let mut maps: Vec<Vec<Guide>> = Vec::new();

        for(si, section) in input_for_day("05")
        .split("\n\n")
        .enumerate() {
            if si == 0 {
                section.split(": ").collect::<Vec<&str>>()[1]
                    .split(' ')
                    .for_each(|n| {
                        seeds.push(n.parse::<u64>().unwrap());
                    });
            }
            else {
                let mut guides = Vec::<Guide>::new();
                section.lines()
                    .filter(|guide| guide.as_bytes()[0].is_ascii_digit())
                    .for_each(|g_line| {
                        let nums = g_line.split(' ')
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>();
                        guides.push(Guide{
                            dest: nums[0],
                            src: nums[1],
                            range: nums[2]
                        });
                });
                maps.push(guides);
            }
    }
    let len = seeds.len();

    maps.iter().for_each(|guides| {
        let mut exeps = Vec::<usize>::new();
        guides.iter().for_each(|guide| {
            for (i, seed) in seeds.iter_mut().enumerate().take(len) {
                if exeps.contains(&i) {
                    continue;
                }

                if *seed >= guide.src && *seed <= (guide.src + guide.range) {
                    let incr_by = *seed - guide.src;
                    *seed = guide.dest + incr_by;
                    exeps.push(i);
                }
            }
        });
    });

    let min = seeds.iter().min().unwrap();
    println!("{}", min);
}
