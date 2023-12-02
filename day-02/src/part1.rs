use std::process::Command;

fn main() {
    let curl_output = Command::new("curl")
        .arg("-X")
        .arg("GET")
        .arg("-H")
        .arg(format!("Cookie: session={}", std::env::var("AOC_SESSION").expect("AOC_SESSION should be set in env")))
        .arg("https://adventofcode.com/2023/day/2/input")
        .output()
        .expect("Cannot download puzzle input");
    let input = String::from_utf8(curl_output.stdout).expect("Cannot parse puzzle input");
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
            println!("Line: {}", line);
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

    const INPUT: &str = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    #[test]
    fn test() {
        let possible_games = get_possible_games(INPUT.to_string(), MAX_CUBES);
        assert!(possible_games == 8);
    }
}

