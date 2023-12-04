advent_of_code::solution!(3);

fn input_into_char_vec(input: &str) -> Vec<Vec<char>> {
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();

        for c in line.chars() {
            row.push(c);
        }

        data.push(row);
    }

    data
}

fn row_check(row: Vec<char>, x: usize, len: usize) -> bool {
    let mut start = 0;
    let mut end = 0;

    if x > 0 { start = x-1 } else { start = x }
    if x+len < row.len()-1 { end = x+len+1 } else { end = x+len }

    for i in start..end {
        if is_symbol(row.get(i).unwrap()) {
            return true
        }
    }

    false
}

// Returns true if a symbol is found around a number
fn look_around_number(data: Vec<Vec<char>>, x: usize, y: usize, len: usize) -> bool{
    if y > 0 {
        if row_check(data.get(y-1).unwrap().clone(), x, len) {
            return true;
        }
    }

    if y < data.len()-1 {
        if row_check(data.get(y+1).unwrap().clone(), x, len) {
            return true;
        }
    }

    let row = data.get(y).unwrap();

    if x > 0 && is_symbol(row.get(x-1).unwrap()) {
        return true
    }

    if x+len < row.len()-1 && is_symbol(row.get(x+len).unwrap()) {
        return true
    }

    false
}

fn is_symbol(c: &char) -> bool {
    if ['0','1','2','3','4','5','6','7','8','9','.'].contains(c) {
        return false
    }

    true
}

fn extract_number(row: Vec<char>, x: usize) -> i32 {
    let mut str = String::new();

    for i in (0..x).rev() {
        let c = row.get(i).unwrap();

        if c.is_digit(10) {
            str.push(*c)
        } else {
            break;
        }
    }

    str = str.chars().rev().collect();

    for i in x..row.len() {
        let c = row.get(i).unwrap();

        if c.is_digit(10) {
            str.push(*c)
        } else {
            break;
        }
    }

    str.parse::<i32>().unwrap()
}

fn row_check_nums(row: Vec<char>, x: usize) -> Vec<i32>{
    let mut numbers: Vec<i32> = Vec::new();

    if row.get(x).unwrap().is_digit(10) {
        numbers.push(extract_number(row.clone(), x));
    }

    if x > 0 && row.get(x-1).unwrap().is_digit(10) {
        numbers.push(extract_number(row.clone(), x-1));
    }

    if x < row.len()-1 && row.get(x+1).unwrap().is_digit(10) {
        numbers.push(extract_number(row.clone(), x+1));
    }

    numbers
}

fn look_around_gear(data: Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();

    if y > 0 {
        numbers.append(&mut row_check_nums(data.get(y - 1).unwrap().to_vec(), x));
    }

    if y < data.len()-1 {
        numbers.append(&mut row_check_nums(data.get(y+1).unwrap().to_vec(), x));
    }

    numbers.append(&mut row_check_nums(data.get(y).unwrap().to_vec(), x));

    numbers.dedup();

    if numbers.len() >= 2 {
        let mut total = 1;

        for number in numbers {
            total *= number;
        }

        return total
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut data: Vec<Vec<char>> = input_into_char_vec(input);
    let mut coords_to_ignore: Vec<(i32,i32)> = Vec::new();

    let mut total = 0;

    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if coords_to_ignore.contains(&(j as i32, i as i32)) {
                continue
            }

            if c.is_digit(10) {
                let mut num: String = c.to_string();

                for k in (j+1)..row.len() {
                    if row.get(k).unwrap().is_digit(10) {
                        num.push(*row.get(k).unwrap());
                        coords_to_ignore.push((k as i32, i as i32))
                    }
                    else { break; }
                }

                if look_around_number(data.clone(), j, i, num.len()) {
                    total += num.parse::<i32>().unwrap()
                }
            }
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut data: Vec<Vec<char>> = input_into_char_vec(input);
    let mut total = 0;

    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if(*c == '*') {
                total += look_around_gear(data.clone(), j, i);
            }
        }
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
