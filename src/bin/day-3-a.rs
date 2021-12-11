use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data/day-3.txt").unwrap();

    let entries = data.lines();

    let mut counts: Vec<i32> = vec![0; 12]; 

    // get char position
    // if 1, +1
    // if 0, -1
    // if > 0, 1 is most common

    for e in entries {
        for (i, c) in e.chars().map(|c| { c.to_digit(10).unwrap() }).enumerate() {
            if c == 1 {
                counts[i] += 1
            } else {
                counts[i] -= 1
            }
        }
    }

    let gamma = counts.iter().map(|c| {
        if c > &0 {
            return "1";
        } else {
            return "0";
        }
    }).collect::<Vec<&str>>().join("");

    let epsilon = gamma.chars().map(|c| if c.to_digit(10).unwrap() == 1 { return "0"; } else { return "1"; }).collect::<Vec<&str>>().join("");

    println!("answer: {}", isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap());
}
