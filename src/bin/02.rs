advent_of_code::solution!(2);

struct Game {
    id: i32,
    blue: i32,
    red: i32,
    green: i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = input.lines();

    let mut id_sum = 0;

    for line in data {
        let mut valid_game = true;

        let mut game = Game {
            id: 0,
            blue: 0,
            red: 0,
            green: 0,
        };

        let mut fields: Vec<&str> = line.split([':', ';', ',']).collect();
        let game_idfield: Vec<&str> = fields.first().unwrap().split_whitespace().collect();
        game.id = game_idfield.last().unwrap().parse::<i32>().unwrap();

        fields.remove(0);

        for field in fields {
            let split_by_whitespace: Vec<&str> = field.split_whitespace().collect();
            let color = split_by_whitespace.last().unwrap().parse::<String>().unwrap();
            let num = split_by_whitespace.first().unwrap().parse::<i32>().unwrap();

            if (color.eq("blue") && num > 14) || (color.eq("red") && num > 12 || color.eq("green") && num > 13) {
                valid_game = false;
                break;
            }
        }

        if valid_game { id_sum += game.id }
    }

    Some(id_sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = input.lines();

    let mut sum = 0;

    for line in data {
        let mut game = Game {
            id: 0,
            blue: 0,
            red: 0,
            green: 0,
        };

        let mut fields: Vec<&str> = line.split([':', ';', ',']).collect();
        fields.remove(0);

        for field in fields {
            let split_by_whitespace: Vec<&str> = field.split_whitespace().collect();
            let color = split_by_whitespace.last().unwrap().parse::<String>().unwrap();
            let num = split_by_whitespace.first().unwrap().parse::<i32>().unwrap();

            if color.eq("blue") && num > game.blue { game.blue = num }
            else if color.eq("red") && num > game.red { game.red = num }
            else if color.eq("green") && num > game.green { game.green = num }
        }

        sum += game.blue * game.red * game.green;
    }

    Some(sum as u32)
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
