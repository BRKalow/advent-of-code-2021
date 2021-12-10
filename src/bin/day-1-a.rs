use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data/day-1.txt").unwrap();

    let entries = data.lines();

    let mut prev: i32 = -1;
    let mut count: i32 = 0;

    for e in entries {
        let int = e.parse::<i32>().unwrap();

        if prev > 0 && int > prev {
            count += 1;
        }

        prev = int;

        println!("{}", e);
    }

    println!("Num increased: {}", count);
}
