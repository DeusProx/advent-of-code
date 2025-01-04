use std::{time::Instant, collections::{VecDeque, HashMap}};

fn main() {
    let input: String = std::fs::read_to_string("../../data/2023/day/17/input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {}", result);
    assert!(result == 1362)
}

fn calc(input: String) -> i32 {
    let now = Instant::now();

    let mut graph = WeightedGraph::parse(input);

    let start = Coords(0, 0);
    let end = Coords(graph.height - 1, graph.width - 1);
    let path_length = graph.shortest_path(start, end, 10);

    println!("path length: {}", path_length);

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    path_length
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords(i32, i32);

impl Coords {
    fn to_index(&self, graph: &WeightedGraph) -> usize {
        (self.0 + self.1 * graph.width) as usize
    }
}

#[derive(Debug, Default)]
struct WeightedGraph {
    width: i32,
    height: i32,
    weights: Vec<i32>,
}

impl WeightedGraph {
    fn parse(input: String) -> Self {
        let width: usize = input.find("\n").unwrap();
        let height: usize = input.len() / (width + 1);
        let weights: Vec<i32> = input.chars()
            .filter(|c| c != &'\n')
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        WeightedGraph { width: width as i32, height: height as i32, weights }
    }
    fn shortest_path(&mut self, start: Coords, end: Coords, max_steps: i32) -> i32 {
        let mut distances: HashMap<(Coords, Coords), i32> = HashMap::new();
        distances.insert((start, Coords(0, 0)), 0);

        let mut steps: VecDeque<(Coords, Coords)> = VecDeque::new();
        steps.push_front((start, Coords(0, 0)));

        loop {
            if steps.is_empty() {
                break;
            }

            let (coords, movement) = steps.pop_front().unwrap();

            for (neighbor_coords, neighbor_step) in self.neighbors((coords, movement), max_steps) {

                let Coords(x, y) = coords;
                if x < 0 || x > self.width as i32 - 1 || y < 0 || y > self.height as i32 - 1 {
                    continue;
                }

                let current_distance = *distances.entry((coords, movement)).or_insert(i32::MAX);

                let neighbor_index = neighbor_coords.to_index(self);
                let neighbor_cost = self.weights.get(neighbor_index).unwrap_or(&i32::MAX);

                let new_distance = current_distance.saturating_add(*neighbor_cost);

                let neighbor_distance = distances.entry((neighbor_coords, neighbor_step)).or_insert(i32::MAX);
                if new_distance < *neighbor_distance {
                    *neighbor_distance = new_distance;
                    steps.push_back((neighbor_coords, neighbor_step));
                }
            }
        }

        *distances.iter()
            .filter(|((coords, _), _)| *coords == end)
            .min_by_key(|(_, distance)| **distance)
            .unwrap()
            .1
    }


    fn neighbors(&self, (Coords(x, y), Coords(sx, sy)): (Coords, Coords), max_steps: i32) -> Vec<(Coords, Coords)> {
        let mut neighbors: Vec<(Coords, Coords)> = Vec::default();

        if 0 < sx && sx < 4 {
            neighbors.push((Coords(x + 1, y), Coords(sx + 1, 0)));
        } else if -4 < sx && sx < 0 {
            neighbors.push((Coords(x - 1, y), Coords(sx - 1, 0)));
        } else if 0 < sy && sy < 4 {
            neighbors.push((Coords(x, y + 1), Coords(0, sy + 1)));
        } else if -4 < sy && sy < 0 {
            neighbors.push((Coords(x, y - 1), Coords(0, sy - 1)));
        } else {
            if sx >= 0 && sx < max_steps {
                neighbors.push((Coords(x + 1, y), Coords(sx + 1, 0)));
            }
            if sx <= 0 && sx > -max_steps {
                neighbors.push((Coords(x - 1, y), Coords(sx - 1, 0)));
            }
            if sy >= 0 && sy < max_steps {
                neighbors.push((Coords(x, y + 1), Coords(0, sy + 1)));
            }
            if sy <= 0 && sy > -max_steps {
                neighbors.push((Coords(x, y - 1), Coords(0, sy - 1)));
            }
        }

        neighbors

        /*
        neighbors.into_iter()
            .filter(|(Coords(x, y), _)| {
                !(*x < 0 || *x > self.width as i32 - 1 || *y < 0 || *y > self.height as i32 - 1)

            })
            .collect()
        */
    }
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/17/testinput").expect("Cannot read input file");
        let result = calc(input);
        assert!(result == 94);
    }
}

