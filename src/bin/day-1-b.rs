use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data/day-1.txt").unwrap();

    let entries = data.lines().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let windows = entries.windows(3);

    let mut prev: i32 = -1;
    let mut count: i32 = 0;

    for e in windows {
        let sum: i32 = e.iter().sum();

        if prev > -1 && sum > prev {
            count += 1;
        }

        prev = sum;
    }

    println!("Num increased: {}", count);
}
