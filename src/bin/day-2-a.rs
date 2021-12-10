use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data/day-2.txt").unwrap();

    let entries = data.lines();

    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for e in entries {
        let v: Vec<&str> = e.split(" ").collect();

        let command: &str = &v[0];
        let value: i32 = v[1].parse().unwrap();

        match command {
          "forward" => horizontal += value,
          "up" => depth -= value,
          "down" => depth += value,
          &_ => ()
        }

        println!("{}", e);
    }

    println!("Final position: {}", depth * horizontal);
}
