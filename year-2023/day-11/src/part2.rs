use std::{fs, time::Instant, str::Chars, iter::repeat};

// Hint: the same as before, but we need i64 instead of i32 and you can set the expansion coefficient in the calc function

fn main() {
    let input: String = fs::read_to_string("./input").expect("Cannot read input file");
    let result = calc(&input, 1_000_000);
    println!("Result: {}", result);
    assert_eq!(result, 731_244_261_352);
}

type Point = (i64, i64);

fn calc(input: &str, expansion_coefficient: i64) -> i64 {
    let now = Instant::now();

    let galaxies = parse_galaxies(input);

    let empty_columns: Vec<i64> = empty_lines(input);
    let empty_rows: Vec<i64> = empty_lines(&transpose(input));
    let galaxies = expand_universe(&galaxies, &empty_columns, &empty_rows, expansion_coefficient);

    let pairs: Vec<(&Point, &Point)> = pairwise(&galaxies);
    let distances: Vec<i64> = pairs.iter()
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
                    '#' => Some((x as i64, y as i64)),
                    _ => None
                })
        })
        .flatten()
        .collect();
    galaxies
}

fn empty_lines(input: &str) -> Vec<i64> {
    let empty_indexes = input.lines().enumerate()
        .filter_map(|(y, line)| match line.contains('#') {
            true => None,
            false => Some(y as i64),
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

fn expand_universe(galaxies: &Vec<Point>, empty_columns: &Vec<i64>, empty_rows: &Vec<i64>, expansion_coefficient: i64) -> Vec<Point> {
    let expand = |index: i64, empty_indexes: &Vec<i64>| {
        index + (expansion_coefficient - 1) * empty_indexes.iter().filter(|&&inner_index| inner_index <= index).count() as i64
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
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let result = calc(&input, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test2() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let result = calc(&input, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn test_universe_expansion() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let galaxies = parse_galaxies(&input);
        let empty_columns = empty_lines(&input);
        let empty_rows = empty_lines(&transpose(&input));

        let galaxies_expanded = expand_universe(&galaxies, &empty_columns, &empty_rows, 2);

        let input_expanded: String = std::fs::read_to_string("./test_expanded").expect("Cannot read input file");
        let galaxies_expanded_input: Vec<Point> = parse_galaxies(&input_expanded);

        assert_eq!(galaxies_expanded, galaxies_expanded_input);
    }
}

