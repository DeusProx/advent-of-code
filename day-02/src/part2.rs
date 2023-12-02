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
    let output = get_power_of_games(input);
    println!("Games possible:\n{output}");
}

#[derive(Default, Debug)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

impl Cubes {
    pub fn min(&self, other: Cubes) -> Cubes {
        Cubes {
            red: i32::max(self.red, other.red),
            green: i32::max(self.green, other.green),
            blue: i32::max(self.blue, other.blue),
        }
    }
}

fn get_power_of_games(input: String) -> i32 {
    input.trim()
        .lines()
        .map(|line| {
            let (_, draws) = line.split_once(": ").expect("Cannot parse line as game");
            let min_set: Cubes = draws.split("; ")
                .map(|draw| {
                    let mut red: i32 = 1;
                    let mut green: i32 = 1;
                    let mut blue: i32 = 1;
                    draw.split(", ").for_each(|drawn_cube| {
                        let cubes = drawn_cube.split(" ").collect::<Vec<&str>>();
                        let cube_amount = cubes[0].parse::<i32>().unwrap();
                        let cube_color = cubes[1];
                        match cube_color {
                            "red" => red = cube_amount,
                            "green" => green = cube_amount,
                            "blue" => blue = cube_amount,
                            _ => panic!("\"{}\" is not red, green or blue", cube_color),
                        }
                    });
                    Cubes { red, green, blue }
                })
                .fold(Cubes::default(), |acc, val|  acc.min(val));
            min_set.red * min_set.green * min_set.blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::get_power_of_games;
    const INPUT: &str = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    #[test]
    fn test() {
        let power_of_games = get_power_of_games(INPUT.to_string());
        assert!(power_of_games == 2286);
    }
}

