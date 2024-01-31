use std::{fs, time::Instant};

fn main() {
    let input: String = fs::read_to_string("./input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {}", result);
    assert_eq!(result, 107142);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Rock::Round => 'O',
            Rock::Cubed => '#',
            Rock::Empty => '.',
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, PartialEq, Eq)]
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
}

// This is probably a really subpar implementation. How to do this correctly?
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().for_each(|column| {
            column.iter().for_each(|rock| {
                let _ = write!(f, "{}", rock);
            });
            let _ = write!(f, "\n");
        });
        Ok(())
    }
}

fn calc(input: String) -> i32 {
    let now = Instant::now();

    let mut grid = Grid::parse(input);
    println!("Grid:\n{}", grid);

    grid.rotate_left();
    grid.tilt();
    println!("Grid:\n{}", grid);

    let load = grid.load();

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    load as i32
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let result = calc(input);
        assert_eq!(result, 136);
    }

}

