use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data/day-3.txt").unwrap();

    let entries: Vec<&str> = data.lines().collect::<Vec<&str>>();
    let counts: Vec<i32> = vec![0; 12]; 

    let mut oxygen_generator = entries.clone();
    let mut c02_scrubber = entries.clone();

      for (i, _) in counts.iter().enumerate() {
        let bit_occurrences = get_bit_occurrences(&oxygen_generator);
        if oxygen_generator.len() == 1 {
          break;
        }

        oxygen_generator = oxygen_generator.into_iter().filter(|s| {
          let nth_char = &s.chars().nth(i).unwrap();
          let most_common_bit = if bit_occurrences[i] < 0 { '0' } else { '1' };

          return nth_char == &most_common_bit;
        }).collect::<Vec<&str>>();
      }

      for (i, _) in counts.iter().enumerate() {
        let bit_occurrences = get_bit_occurrences(&c02_scrubber);
        if c02_scrubber.len() == 1 {
          break;
        }

        c02_scrubber = c02_scrubber.into_iter().filter(|s| {
          let nth_char = &s.chars().nth(i).unwrap();
          let least_common_bit = if bit_occurrences[i] < 0 { '1' } else { '0' };

          return nth_char == &least_common_bit;
        }).collect::<Vec<&str>>();
      }

    println!("result: {}, {}", &oxygen_generator[0], &c02_scrubber[0]);

    println!("{}", isize::from_str_radix(&oxygen_generator[0], 2).unwrap() * isize::from_str_radix(&c02_scrubber[0], 2).unwrap());
}

fn get_bit_occurrences(binary_strings: &Vec<&str>) -> Vec<i32> {
  let mut counts: Vec<i32> = vec![0; 12]; 

    // get char position
    // if 1, +1
    // if 0, -1
    // if > 0, 1 is most common

    for e in binary_strings.clone() {
        for (i, c) in e.chars().map(|c| { c.to_digit(10).unwrap() }).enumerate() {
            if c == 1 {
                counts[i] += 1
            } else {
                counts[i] -= 1
            }
        }
    }

    return counts;
}
