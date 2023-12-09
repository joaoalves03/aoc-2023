advent_of_code::solution!(7);

struct Hand {
    scores: Vec<u8>,
    bid: u32
}

#[derive(PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_to_score(card: &char, joker: bool) -> u8 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => if joker {0} else {10},
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0
    }
}

fn parse_hand(raw_hand: &str, joker: bool) -> Hand {
    let fields: Vec<&str> = raw_hand.split_whitespace().collect();

    let mut hand = Hand {
        scores: vec![],
        bid: fields.last().unwrap().parse::<u32>().unwrap(),
    };

    for card in fields.first().unwrap().chars() {
        hand.scores.push(card_to_score(&card, joker));
    }

    hand
}

fn get_hand_type(scores: &Vec<u8>, joker: bool) -> HandType {
    let mut scores_without_duplicates = scores.clone();

    if joker {
        scores_without_duplicates = scores_without_duplicates
            .into_iter()
            .filter(|&x| x != 0)
            .collect::<Vec<u8>>();
    }

    scores_without_duplicates.sort();
    scores_without_duplicates.dedup();

    match scores_without_duplicates.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            for score in scores_without_duplicates {
                let mut instances_of_score = scores.iter().filter(|&&x| (x == score || x == 0)).count();

                if instances_of_score == 4 {
                     return HandType::FourOfAKind
                }
            }

            return HandType::FullHouse
        }
        3 => {
            for score in scores_without_duplicates {
                let instances_of_score = scores.iter().filter(|&&x| (x == score || x == 0)).count();

                if instances_of_score == 3 {
                    return HandType::ThreeOfAKind
                }
            }

            return HandType::TwoPair
        }
        4 => HandType::OnePair,
        _ => HandType::HighCard
    }
}

fn sort_hands(hands: &mut Vec<Hand>, hand_types: &mut Vec<HandType>) {
    let mut swapped;
    let len = hands.len();

    loop {
        swapped = false;

        for i in 0..len-1 {
            let left_hand_type = hand_types.get(i).unwrap();
            let right_hand_type = hand_types.get(i+1).unwrap();

            if left_hand_type == right_hand_type {
                let left_hand = hands.get(i).unwrap();
                let right_hand = hands.get(i+1).unwrap();

                for (j, score) in left_hand.scores.iter().enumerate() {
                    let right_score = right_hand.scores.get(j).unwrap();

                    if score > right_score {
                        hand_types.swap(i,i+1);
                        hands.swap(i, i+1);
                        swapped = true;
                        break;
                    } else if score < right_score {
                        break;
                    }
                }

                continue;
            }

            if left_hand_type > right_hand_type {
                hand_types.swap(i, i+1);
                hands.swap(i, i+1);
                swapped = true;
            }
        }

        if !swapped { break; }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|x| parse_hand(x, false)).collect();
    let mut hand_types: Vec<HandType> = hands.iter()
        .map(|hand| get_hand_type(&hand.scores, false)).collect();

    sort_hands(&mut hands, &mut hand_types);

    let mut total = 0;

    for (i, hand) in hands.iter().enumerate() {
        total += (i+1) as u32 * hand.bid;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|x| parse_hand(x, true)).collect();
    let mut hand_types: Vec<HandType> = hands.iter()
        .map(|hand| get_hand_type(&hand.scores, true)).collect();

    sort_hands(&mut hands, &mut hand_types);

    let mut total = 0;

    for (i, hand) in hands.iter().enumerate() {
        total += (i+1) as u32 * hand.bid;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
