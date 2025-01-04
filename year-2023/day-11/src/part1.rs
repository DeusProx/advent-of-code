use std::{time::Instant, str::Chars, iter::repeat};

// Hint: Manhatten distance can be used to calc the distance between the galaxies

fn main() {
    let input: String = std::fs::read_to_string("../../data/2023/day/11/input").expect("Cannot read input file");
    let result = calc(&input);
    println!("Result: {}", result);
    assert_eq!(result, 9_723_824);
}

type Point = (i32, i32);

fn calc(input: &str) -> i32 {
    let now = Instant::now();

    let galaxies = parse_galaxies(input);

    let empty_columns: Vec<i32> = empty_lines(input);
    let empty_rows: Vec<i32> = empty_lines(&transpose(input));
    let galaxies = expand_universe(&galaxies, &empty_columns, &empty_rows, 2);

    let pairs: Vec<(&Point, &Point)> = pairwise(&galaxies);
    let distances: Vec<i32> = pairs.iter()
        .map(|((x1, y1), (x2, y2))| (x1 - x2).abs() + (y1 - y2).abs()) // manhatten distance
        .collect();

    let elapsed = now.elapsed();
    println!("Time: {} μs (~{} ms)", elapsed.as_micros(), elapsed.as_millis());

    distances.iter().sum()
}

fn parse_galaxies(input: &str) -> Vec<Point> {
    let galaxies = input.lines().enumerate()
        .map(|(y, line)| {
            line.chars().enumerate()
                .filter_map(move |(x, c)| match c {
                    '#' => Some((x as i32, y as i32)),
                    _ => None
                })
        })
        .flatten()
        .collect();
    galaxies
}

fn empty_lines(input: &str) -> Vec<i32> {
    let empty_indexes = input.lines().enumerate()
        .filter_map(|(y, line)| match line.contains('#') {
            true => None,
            false => Some(y as i32),
        })
        .collect();
    empty_indexes
}

fn transpose(input: &str) -> String {
    let mut iters: Vec<Chars<'_>> = input.lines().map(|line| line.chars()).collect();

    let transposed = std::iter::from_fn(|| {
        let mut index = 0;

        let row = std::iter::from_fn(|| {
            let iter = iters.get_mut(index);
            index += 1;

            match iter.is_none() {
                true => None,
                false => iter.unwrap().next(),
            }
        }).collect::<String>();

        match row.is_empty() {
            true => None,
            false => Some(row + "\n"),
        }
    }).collect::<String>();

    transposed
}

fn expand_universe(galaxies: &Vec<Point>, empty_columns: &Vec<i32>, empty_rows: &Vec<i32>, expansion_coefficient: i32) -> Vec<Point> {
    let expand = |index: i32, empty_indexes: &Vec<i32>| {
        index + (expansion_coefficient - 1) * empty_indexes.iter().filter(|&&inner_index| inner_index <= index).count() as i32
    };
    let exp = galaxies.iter()
        .map(|&(x, y)| (
            expand(x, empty_rows),
            expand(y, empty_columns),
        ))
        .collect();
    exp
}

fn pairwise<T>(elements: &Vec<T>) -> Vec<(&T, &T)> {
    let pairs = elements.iter()
        .enumerate()
        .flat_map(|(index, element)| {
            repeat(element)
                .zip(elements.iter().skip(index + 1))
                .collect::<Vec<(&T, &T)>>()
        })
        .collect();
    pairs
}

#[cfg(test)]
mod tests {
    use crate::{calc, Point, parse_galaxies, empty_lines, transpose, expand_universe};

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/11/testinput").expect("Cannot read input file");
        let result = calc(&input);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_universe_expansion() {
        let input: String = std::fs::read_to_string("../../data/2023/day/11/testinput").expect("Cannot read input file");
        let galaxies = parse_galaxies(&input);
        let empty_columns = empty_lines(&input);
        let empty_rows = empty_lines(&transpose(&input));

        let galaxies_expanded = expand_universe(&galaxies, &empty_columns, &empty_rows, 2);

        let input_expanded: String = std::fs::read_to_string("../../data/2023/day/11/testinput_expanded").expect("Cannot read input file");
        let galaxies_expanded_input: Vec<Point> = parse_galaxies(&input_expanded);

        assert_eq!(galaxies_expanded, galaxies_expanded_input);
    }
}

