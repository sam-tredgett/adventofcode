use std::thread::current;

#[derive(Debug, Clone, Copy, PartialEq)]
struct PartNumber {
    value: i32,
    valid: bool,
}

fn main() {
    let input = include_str!("../input.txt");
    let part_numbers = parse_numbers_from_string_slice(input); 
    let mut accum = 0;
    for part_number in part_numbers {
        if part_number.valid {
            accum = accum + part_number.value;
        }
    }
    println!("Accumulative value of valid part number is {}", accum)
}

fn parse_numbers_from_string_slice(input: &str) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut iter = input.lines().map(|line| line.trim()).enumerate().into_iter();
   
    let mut prev_line: &str = "";
    let mut current_line: &str = ""; 
    let mut next_line: &str = ""; 
    while let Some((_index, new_line)) = iter.next() {
        prev_line = current_line;
        current_line = next_line;
        next_line = new_line;
        if current_line == "" { continue };

        println!("\nprevious_line= {}",prev_line);
        println!("current_line= {}", current_line);
        println!("next_line= {}", next_line);

        if let Some(found_part_numbers) = get_part_number_from_iteration(prev_line, current_line, next_line) {
            found_part_numbers.iter().for_each(|&fpn| part_numbers.push(fpn));
        }

    }
    return part_numbers;
}

fn get_part_number_from_iteration(prev_line: &str, current_line: &str, next_line: &str ) -> Option<Vec<PartNumber>> {
    let _p: PartNumber = PartNumber {value: 0, valid: false};
    let mut num_str: Vec<char>= vec![];
    for (i , c) in current_line.chars().enumerate() {
            if c.is_numeric() {
                num_str.push(c);
            } else if num_str.len() > 0 || i == current_line.len()-1 {
                println!("found num_str={}", num_str.iter().collect::<String>());
                num_str.clear();
            } 
        }

    None
}


#[cfg(test)]
mod tests {
    use crate::parse_numbers_from_string_slice;
    use crate::PartNumber;
    #[test]
    fn test_gets_valid_part_numbers() {
        let test = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        
        let part_numbers: Vec<PartNumber> =  vec![
                                     PartNumber {value: 467, valid: true },
                                     PartNumber {value: 114, valid: false},
                                     PartNumber {value: 35, valid: true },
                                     PartNumber {value: 633, valid: true },
                                     PartNumber {value: 617, valid: true },
                                     PartNumber {value: 58, valid: false},
                                     PartNumber {value: 592, valid: true},
                                     PartNumber {value: 755, valid: true },
                                     PartNumber {value: 644, valid: true },
                                     PartNumber {value: 598, valid: true },
        ];
        let results = parse_numbers_from_string_slice(test);
        for (test, result) in (part_numbers, results).try_into().iter() {
            assert_eq!(test, result);
        }
    }
}