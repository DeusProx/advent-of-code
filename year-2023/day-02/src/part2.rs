use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/2/input").expect("Cannot read input file");
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

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/2/testinput").expect("Cannot read input file");
        let power_of_games = get_power_of_games(input.to_string());
        assert!(power_of_games == 2286);
    }
}

