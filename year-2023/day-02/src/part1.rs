use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/2/input").expect("Cannot read input file");
    let output = get_possible_games(input, MAX_CUBES);
    println!("Games possible:\n{output}");
}

struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}
const MAX_CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn get_possible_games(input: String, max_cubes: Cubes) -> i32 {
    input.trim()
        .lines()
        .map(|mut line| {
            line = line.trim().strip_prefix("Game ").unwrap();
            let (game, draws) = line.split_once(": ").expect("Cannot parse line as a game");
            let game_number = game.parse::<i32>().unwrap();

            let is_game_possible = draws.split("; ")
                .all(|draw| {
                    draw.split(", ").all(|drawn_cube| {
                        let cubes = drawn_cube.split(" ").collect::<Vec<&str>>();
                        let cube_amount = cubes[0].parse::<i32>().unwrap();
                        let cube_color = cubes[1];
                        match cube_color {
                            "red" => cube_amount <= max_cubes.red,
                            "green" => cube_amount <= max_cubes.green,
                            "blue" => cube_amount <= max_cubes.blue,
                            _ => panic!("\"{}\" is not red, green or blue", cube_color),
                        }
                    })
                });

            match is_game_possible {
                true => game_number,
                false => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{get_possible_games, MAX_CUBES};

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/2/testinput").expect("Cannot read input file");
        let possible_games = get_possible_games(input.to_string(), MAX_CUBES);
        assert!(possible_games == 8);
    }
}

