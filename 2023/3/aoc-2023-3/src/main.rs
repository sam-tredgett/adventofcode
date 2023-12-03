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
        println!("found part_number with value={} and valid={}", part_number.value, part_number.valid);
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
        if let Some(found_part_numbers) = get_part_number_from_iteration(prev_line, current_line, next_line) {
            found_part_numbers.iter().for_each(|&fpn| part_numbers.push(fpn));
        }
    }

    prev_line = current_line;
    current_line = next_line;
    next_line = "";
    if let Some(found_part_numbers) = get_part_number_from_iteration(prev_line, current_line, next_line) {
        found_part_numbers.iter().for_each(|&fpn| part_numbers.push(fpn));
    }

    return part_numbers;
}

fn get_part_number_from_iteration(prev_line: &str, current_line: &str, next_line: &str ) -> Option<Vec<PartNumber>> {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut num_str: Vec<char>= vec![];
    let mut num_str_beginning_index = 0;
    let num_str_ending_index = 0;
    for (i , c) in current_line.chars().enumerate() {
        if c.is_numeric() {
            if num_str.len() == 0 {
                num_str_beginning_index = i;
            }
            num_str.push(c);

            if i+1 == current_line.len() {
                let part_num = convert_num_str_to_part_num(num_str.iter().collect::<String>(), num_str_beginning_index, num_str_ending_index, prev_line, current_line, next_line, i);
                part_numbers.push(part_num);
            }
        } else if num_str.len() > 0 || i == current_line.len()-1 {
            let found_num = num_str.iter().collect::<String>();                                                       
            if found_num != "" {
               let part_num =  convert_num_str_to_part_num(found_num, num_str_beginning_index, num_str_ending_index, prev_line, current_line, next_line, i);
                part_numbers.push(part_num);
            }

            num_str.clear();
        } 
    }
    if part_numbers.len() > 0 {
        return Some(part_numbers);
    }
    None
}

fn convert_num_str_to_part_num(
    found_num: String, 
    num_str_beginning_index: usize, 
    num_str_ending_index: usize,
    prev_line: &str,
    current_line: &str,
    next_line: &str,
    current_index:usize
                               ) -> PartNumber {
    let mut part_num = PartNumber { 
        value:found_num.parse().unwrap_or(0), 
        valid:false
    };

    let peek_start_index = if num_str_beginning_index > 0 { num_str_beginning_index - 1 } else {num_str_beginning_index};
    let peek_end_index = if num_str_ending_index >= current_line.len() { current_index } else {current_index +1 };

    if peek_end_index < current_line.len() { // Our last digit not EOL
        let next = current_line.chars().collect::<Vec<char>>()[peek_end_index-1];
        if  !next.is_alphanumeric() && next != '.' {
            part_num.valid = true; 
        }   
    }

    if peek_start_index > 0  && !part_num.valid { // Our first digit not start of line
        let prev = current_line.chars().collect::<Vec<char>>()[peek_start_index];
        if !prev.is_alphanumeric() && prev != '.' {
            part_num.valid = true;
        }
    }

    if !part_num.valid && prev_line != "" {
        for c in prev_line[peek_start_index..peek_end_index].chars().collect::<Vec<char>>().iter() {
            if !c.is_alphanumeric() &&  *c != '.' {
                part_num.valid = true; 
                break;
            }
        }
    }

    if !part_num.valid && next_line != "" {
        for c in next_line[peek_start_index..peek_end_index].chars().collect::<Vec<char>>().iter() {
            if !c.is_alphanumeric() &&  *c != '.'{
                part_num.valid = true; 
                break;
            }
        }
    }
    return part_num;
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
        .664.598..
        .....*....
        ......321.";
        
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
                                     PartNumber {value: 321, valid: true },

        ];
        let results = parse_numbers_from_string_slice(test);
        for (test, result) in (part_numbers, results).try_into().iter() {
            assert_eq!(test, result);
        }
    }
}
