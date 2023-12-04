advent_of_code::solution!(4);

struct ScratchCard {
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>
}

fn extract_numbers(line: &str, num: usize) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    let strings = line
        .split(&[':', '|'])
        .nth(num).unwrap().trim()
        .split_whitespace();

    for str in strings {
        numbers.push(str.parse::<i32>().unwrap())
    }

    numbers
}

fn get_scratch_cards(input: &str) -> Vec<ScratchCard> {
    let mut scratch_cards: Vec<ScratchCard> = Vec::new();

    for line in input.lines() {
        let scratch_card = ScratchCard {
            winning_numbers: extract_numbers(line, 1),
            numbers: extract_numbers(line, 2),
        };

        scratch_cards.push(scratch_card)
    }

    scratch_cards
}

fn get_number_of_cards(i: usize, cards: &Vec<ScratchCard>) -> i32 {
    let mut number_of_cards = 0;

    for num in &cards.get(i).unwrap().winning_numbers {
        if cards.get(i).unwrap().numbers.contains(&num) {
            number_of_cards += 1;
        }
    }

    if number_of_cards > 0 {
        for j in i + 1..i + number_of_cards + 1 {
            number_of_cards += get_number_of_cards(j, cards) as usize;
        }
    }

    number_of_cards as i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let scratch_cards: Vec<ScratchCard> = get_scratch_cards(input);
    let mut total = 0;

    for card in scratch_cards {
        let mut score = 0;

        for num in card.winning_numbers {
            if card.numbers.contains(&num) {
                if score == 0 { score = 1 }
                else { score *= 2}
            }
        }

        total += score
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let scratch_cards: Vec<ScratchCard> = get_scratch_cards(input);
    let mut total = 0;

    for i in 0..scratch_cards.len() {
        total += get_number_of_cards(i, &scratch_cards) + 1;
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
