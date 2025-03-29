use std::{sync::{atomic::AtomicUsize, Arc}, time::Instant};
use rayon::prelude::*;

// Note: The whole maze isn't really a maze, but one long path

#[aoc_macro::bench()]
pub fn part1() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/20/input").expect("Cannot read input"); // 1351

    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);
    let mut maze: Vec<char> = input.chars().filter(|&c| c != '\n').collect();

    let mut start = input.find('S').unwrap();
    start = start - start / (width + 1);
    let mut end = input.find('E').unwrap();
    end = end - end / (width + 1);

    maze[start] = '.';
    maze[end] = '.';

    let mut queue = vec![start];
    let mut costs = vec![usize::MAX; width * height];
    let _steps = dfs(&mut maze.clone(), &mut costs, width, height, &mut queue, end, 0);

    // Since we can just cheat for two picoseconds we can only jump one wall
    // -> Find all walls which neighbors belong to the path
    let cheat_savings: Vec<usize> = maze.iter().enumerate()
        .filter_map(|(index, &c)| (c == '#').then(|| index))
        .filter_map(|index| {
            let mut elements: Vec<usize> = get_neighbors(width, height, index)
                .filter(|&index| maze[index] == '.')
                .map(|index| costs[index])
                .collect();
            if elements.len() < 2 {
                return None;
            }
            elements.sort();

            let cheat_saving = elements[elements.len() - 1] - elements[0] - 2;
            Some(cheat_saving)
        })
        // .filter(|&cheat_saving| cheat_saving > 0) // for testinput
        .filter(|&cheat_saving| cheat_saving > 99) // for real input
        .collect();

    cheat_savings.len()
}

pub fn dfs(maze: &mut Vec<char>, costs: &mut Vec<usize>, w: usize, h: usize, queue: &mut Vec<usize>, end: usize, step: usize) -> Option<usize> {
    let Some(current) = queue.pop() else {
        return None;
    };
    if current == end {
        costs[current] = step;
        maze[current] = 'x';
        return Some(step);
    }

    costs[current] = step;
    let step = step + 1;
    maze[current] = 'x';

    let elements = get_neighbors(w, h, current)
        .filter(|&index| maze[index] == '.');

    // add neighbors to queue
    queue.extend(elements);

    dfs(maze, costs, w, h, queue, end, step)
}

pub fn get_neighbors(w: usize, h: usize, current: usize) -> impl Iterator<Item = usize> {
    let (x, y) = (current % w, current / w);
    [
        (y > 0      ).then_some((y - 1) * w + x    ), // north
        (y < (h - 1)).then_some((y + 1) * w + x    ), // south
        (x > 0      ).then_some( y      * w + x - 1), // west
        (x < (w - 1)).then_some( y      * w + x + 1), // east
    ]
        .into_iter()
        .flatten()
}


#[aoc_macro::bench()]
pub fn part2() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/20/input").expect("Cannot read input"); // 966130

    let width = input.find('\n').unwrap();
    let height = input.len() / (width + 1);
    let mut maze: Vec<char> = input.chars().filter(|&c| c != '\n').collect();

    let mut start = input.find('S').unwrap();
    start = start - start / (width + 1);
    let mut end = input.find('E').unwrap();
    end = end - end / (width + 1);

    maze[start] = '.';
    maze[end] = '.';

    let mut queue = vec![start];
    let mut costs = vec![usize::MAX; width * height];
    let _steps = dfs(&mut maze.clone(), &mut costs, width, height, &mut queue, end, 0);

    let path_iterator: Vec<usize> = maze.iter()
        .enumerate()
        .filter_map(|(index, &c)| (c != '#').then(|| index))
        .collect();

    let solution: Solution = Solution::Loops;
    let check_cheat = |a: usize, b: usize| -> Option<usize> {
        let (x_1, y_1) = (a % width, a / width);
        let (x_2, y_2) = (b % width, b / width);

        let cheat_steps = x_1.abs_diff(x_2) + y_1.abs_diff(y_2);
        if cheat_steps >= 21 {
            return None
        }

        let savings = costs[a].abs_diff(costs[b]) - cheat_steps;
        // if savings < 50 { // for testinput
        if savings < 100 { // for real input
            return None
        }

        Some(savings)
    };

    match solution {
        Solution::Iterator => { // ~125 ms
            let cheat_iterator = path_iterator.iter()
                .enumerate()
                .flat_map(|(index, &a)| path_iterator.iter()
                    .skip(index + 1)
                    .map(move |&b| (a, b))
                );

            let cheat_savings: Vec<usize> = cheat_iterator
                .filter_map(|(a, b)| check_cheat(a, b))
                .collect();

            cheat_savings.len()
        },
        Solution::Loops => { // ~60 ms with for loops
            let mut counter = 0;
            let mut count = || counter += 1;
            for (index, &a) in path_iterator.iter().enumerate() {
                for &b in path_iterator.iter().skip(index + 1) {
                    if let Some(_) = check_cheat(a, b) {
                        count();
                    };
                }
            }
            counter
        },
        Solution::Rayon => { // ~13ms with rayon and Arc<AtomicUsize>
            let cheat_counter: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
            let count_fn = || cheat_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            path_iterator.clone().into_par_iter().enumerate().for_each(|(index, a)| {
                for &b in path_iterator.iter().skip(index + 1) {
                    if let Some(_) = check_cheat(a,b) {
                        count_fn();
                    };
                }
            });
            cheat_counter.load(std::sync::atomic::Ordering::SeqCst)
        },
    }

}

pub enum Solution {
    Iterator,
    Loops,
    Rayon,
}
