use std::{fs, time::Instant, collections::HashMap};

fn main() {
    let input: String = fs::read_to_string("./input").expect("Cannot read input file");
    let result = calc(input, 1_000_000_000);
    println!("Result: {}", result);
    assert_eq!(result, 104815);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cubed,
    Empty,
}

impl Rock {
    fn parse(c: char) -> Self {
        match c {
            'O' => Rock::Round,
            '#' => Rock::Cubed,
            '.' => Rock::Empty,
            _ => panic!("Nooo"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<Rock>>
}

impl Grid {
    fn parse(input: String) -> Self {
        let width = input.find("\n").unwrap();
        let height = input.len() / (width + 1);
        let data: Vec<Vec<Rock>> = input.lines()
            .map(|line| line
                .chars()
                .map(Rock::parse)
                .collect::<Vec<Rock>>()
            )
            .collect();
        Self { width, height, data }
    }

    fn rotate_right(&mut self) {
        (self.width, self.height) = (self.height, self.width);
        let mut iters: Vec<_> = self.data.iter().map(|column| column.iter()).collect();
        self.data = (0..self.width)
                .map(|_| iters
                    .iter_mut()
                    .map(|column| *column.next().unwrap())
                    .rev()
                    .collect::<Vec<Rock>>()
                )
                .collect();
    }

    fn rotate_left(&mut self) {
        (self.width, self.height) = (self.height, self.width);
        let mut iters: Vec<_> = self.data.iter().map(|column| column.iter()).collect();
        self.data = (0..self.width)
            .map(|_| iters
                .iter_mut()
                .map(|column| *column.next_back().unwrap())
                .collect::<Vec<Rock>>()
            )
            .collect::<Vec<_>>()
    }

    fn tilt(&mut self) {
        for column in self.data.iter_mut() {
            let mut pivot_index = 0;
            for i in 0..column.len() {
                match column[i] {
                    Rock::Cubed => pivot_index = i + 1,
                    Rock::Round => {
                        (column[i], column[pivot_index]) = (column[pivot_index], column[i]);
                        pivot_index += 1;
                    },
                    _ => continue,
                }
            }
        }
    }

    fn load(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(_, column)| {
                column.iter()
                    .enumerate()
                    .map(|(x, rock)| {
                        match *rock {
                            Rock::Round => column.len() - x,
                            _ => 0,
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt();
            self.rotate_right();
        }
    }
}

fn calc(input: String, cycles: i32) -> i32 {
    let now = Instant::now();

    let mut grid = Grid::parse(input.clone());
    grid.rotate_left();

    let mut seen: HashMap<Grid, i32> = HashMap::new();
    let mut current_step = 0;

    loop {
        if seen.contains_key(&grid) {
            break;
        }
        seen.insert(grid.clone(), current_step);

        current_step += 1;
        grid.cycle();
    }

    let step_of_repetition = seen.get(&grid).unwrap();
    let steps_until_repetition = current_step - step_of_repetition;
    let remaining = (cycles - current_step) % steps_until_repetition;

    for _ in 0..remaining {
        grid.cycle();
    }
    let load = grid.load();

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    load as i32
}

#[cfg(test)]
mod tests {
    use crate::{calc, Grid};

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let result = calc(input, 1_000_000_000);
        assert_eq!(result, 64);
    }

    #[test]
    fn rotation_test() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");

        let mut grid = Grid::parse(input.clone());
        grid.rotate_right();
        grid.rotate_right();
        grid.rotate_right();

        let mut grid2 = Grid::parse(input);
        grid2.rotate_left();

        assert_eq!(grid, grid2);
    }
}

