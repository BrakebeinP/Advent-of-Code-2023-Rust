use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let strings: Vec<&str> = input.split("\n").collect();

    let mut total: u32 = 0;

    for s in strings {
        let mut first: String = "".to_string();
        let mut last: String = "".to_string();
        for chr in s.chars() {
            if chr.is_digit(10) && first == "" {
                first = chr.to_string();
            } else if chr.is_digit(10) {
                last = chr.to_string();
            }
        }
        if last == "".to_string() {
            last = first.clone();
        }
        let num_str: String = [first, last].join("");
        let num = num_str.parse::<u32>();
        match num {
            Ok(n) => total += n,
            Err(n) => println!("error {}", n),
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let strings: Vec<&str> = input.split("\n").collect();

    let mut total: u32 = 0;

    let num_conversion: HashMap<&str, &str> = HashMap::from([
        ("one", "on1e"),
        ("two", "tw2o"),
        ("three", "thre3e"),
        ("four", "fo4our"),
        ("five", "fi5ve"),
        ("six", "si6x"),
        ("seven", "se7ven"),
        ("eight", "ei8ght"),
        ("nine", "ni9ne"),
    ]);

    for s in strings {
        let mut replaced_str = s.to_string();
        let mut first: String = "".to_string();
        let mut last: String = "".to_string();

        for (k, v) in num_conversion.clone().iter() {
            replaced_str = replaced_str.replace(k, v);
        }

        for chr in replaced_str.chars() {
            if chr.is_digit(10) && first == "" {
                first = chr.to_string();
            } else if chr.is_digit(10) {
                last = chr.to_string();
            }
        }

        if last == "".to_string() {
            last = first.clone();
        }
        let num_str: String = [first, last].join("");
        let num = num_str.parse::<u32>();
        match num {
            Ok(n) => total += n,
            Err(n) => println!("error {}", n),
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
