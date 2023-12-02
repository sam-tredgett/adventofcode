fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines().map(|line| line.trim()); 
    let mut accum = 0; 
    for line in lines{
        let curr = get_num_from_line(line);  
        accum  += curr;
        println!("Line={} contained={} running total={}", line, curr, accum)
    }
}

fn match_substring_to_digit(substring: &str) -> Option<String> {
    let digits = vec![
                                        ("one", String::from("1")),
                                        ("two", String::from("2")),
                                        ("three", String::from("3")),
                                        ("four", String::from("4")),
                                        ("five", String::from("5")),
                                        ("six", String::from("6")),
                                        ("seven", String::from("7")),
                                        ("eight", String::from("8")),
                                        ("nine", String::from("9"))
                                        ];

    let mut matches: Vec<String> = vec![];
    for digit in digits {
        if substring.to_lowercase().contains(digit.0) {
            matches.push(digit.1);
        }
    }
    let len = &matches.len();
    if len.clone() > 0 {
        let last_num = &matches[len.clone()-1];
        return Some(last_num.to_string());
    }
    None
}

fn get_num_from_line(line: &str) -> i32 {
    let mut found_nums: Vec<String> = vec![];
    let my_chars: Vec<_> = line.chars().collect();
    let mut sub_str = String::new(); 
    my_chars.iter()
            .for_each(|&c| {
                if c.is_numeric() {
                    found_nums.push(c.to_string());
                    sub_str.clear();
                } else {
                    sub_str.push(c);
                    let mut matches: Vec<String> = vec![];
                    let mut index = 0;
                    while index != sub_str.len() {
                        let reduced_string = &sub_str[index..];
                        let num = match_substring_to_digit(reduced_string);
                        match num {
                            Some(x) => matches.push(x),
                            None => () 
                        }
                        index = index + 1;
                    }
                    if matches.len() > 0 {
                        let last_num  = &matches[matches.len()-1];
                        found_nums.push(last_num.to_string());
                    }
                }
            });

    let mut final_num_str = String::new();
    match found_nums.len() {
        1 =>  final_num_str = format!("{}{}", found_nums[0], found_nums[0]),
        0 => (),
        _ => final_num_str = format!("{}{}", found_nums[0], found_nums[found_nums.len()-1]),
    }
    let num: i32 = final_num_str.parse().unwrap();
    num
}

#[cfg(test)]
mod tests {
    use super::get_num_from_line;
    #[test]
    fn gets_digits() {
        let tests = 
                vec![
                     ("1abc2", 12),
                     ("pqr3stu8vwx", 38), 
                     ("a1b2c3d4e5f", 15),
                     ("treb7uchet", 77),
                     ("two1nine", 29),
                     ("eightwothree", 83),
                     ("abcone2threexyz", 13),
                     ("xtwone3four", 24),
                     ("4nineeightseven2", 42),
                     ("zoneight234", 14),
                     ("7pqrstsixteen", 76),
                     ("eightjbsfdh5threesevenfzgqpxfvkghzntfrplpg7oneighthh", 88)
                     ]; 

        for test in tests {
            let value = get_num_from_line(test.0);
            println!("left={}, right={}", &value, test.1);
            assert_eq!(value,test.1);
        } 
    }
}