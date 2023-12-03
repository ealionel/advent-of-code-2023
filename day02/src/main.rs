use std::fs;

#[derive(Debug)]
struct GrabSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GrabSet>,
}

fn parse_line(line: &str) -> Game {
    let parts: Vec<&str> = line.split(": ").collect();
    let (game_info, sets_info) = (parts[0], parts[1]);

    let game_id: u32 = game_info
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    let sets: Vec<GrabSet> = sets_info
        .split("; ")
        .map(|set: &str| {
            let colors_info = set.split(", ");

            let mut grab_set: GrabSet = GrabSet {
                red: 0,
                green: 0,
                blue: 0,
            };

            colors_info.for_each(|color_info| {
                let color_parts: Vec<&str> = color_info.split(" ").collect();
                let (amount, color) = (color_parts[0].parse::<u32>().unwrap(), color_parts[1]);

                match color {
                    "red" => grab_set.red = amount,
                    "blue" => grab_set.blue = amount,
                    "green" => grab_set.green = amount,
                    _ => {
                        println!("Unknown color: {}", color)
                    }
                }
            });

            grab_set
        })
        .collect();

    let game: Game = Game {
        id: game_id,
        sets: sets,
    };

    return game;
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let games = input.lines().map(|line| parse_line(line));

    const MIN_RED: u32 = 12;
    const MIN_GREEN: u32 = 13;
    const MIN_BLUE: u32 = 14;

    let answer1 = games
        .clone()
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| set.red <= MIN_RED && set.green <= MIN_GREEN && set.blue <= MIN_BLUE)
        })
        .fold(0, |acc, game| acc + game.id);

    let answer2 = games
        .clone()
        .map(|game| {
            let red = game.sets.iter().map(|set| set.red).max().unwrap();
            let green = game.sets.iter().map(|set| set.green).max().unwrap();
            let blue = game.sets.iter().map(|set| set.blue).max().unwrap();

            red * green * blue
        })
        .fold(0, |acc, value| acc + value);

    println!("Answer 1: {}", answer1,);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "Game 24: 1 green, 2 blue; 13 red, 2 blue, 3 green; 4 green, 14 red";
        let game = parse_line(input);

        println!("{:?}", game.sets);
        assert_eq!(game.id, 24);
        assert_eq!(game.sets.len(), 3);
    }
}
