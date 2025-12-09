use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/9/input").expect("Cannot read input");

    let corners: Vec<(u64, u64)> = input.lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)|(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .collect();

    let mut max = u64::MIN;
    for i in 0..corners.len() {
        let a = corners[i];
        for j in (i + 1)..corners.len() {
            let b = corners[j];
            let area = ((a.0).abs_diff(b.0) + 1) * ((a.1).abs_diff(b.1) + 1);
            if area > max {
                max = area;
            }
        }
    }

    max
}

#[aoc_macro::bench()]
pub fn part2() -> u64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/9/input").expect("Cannot read input");

    let corners: Vec<(u64, u64)> = input.lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)|(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .collect();

    let mut areas: Vec<(u64, (usize, usize))> = Vec::with_capacity(corners.len() * (corners.len() - 1) / 2);
    for i in 0..corners.len() {
        let a = corners[i];
        for j in (i + 1)..corners.len() {
            let b = corners[j];
            let area = ((a.0).abs_diff(b.0) + 1) * ((a.1).abs_diff(b.1) + 1);
            areas.push((area, (i, j)));
        }
    }
    areas.sort_by_key(|&(area, _)| area);

    let mut h_edges: Vec<((u64, u64, u64),(usize, usize))> = Vec::with_capacity(corners.len());
    let mut v_edges: Vec<((u64, u64, u64),(usize, usize))> = Vec::with_capacity(corners.len());
    for i in 0..corners.len() {
        let a = corners[i];
        let j = (i + 1) % corners.len();
        let b = corners[j];

        if a.0 == b.0 {
            match a.1 < b.1 {
                true  => v_edges.push(((a.0, a.1, b.1), (i, j))),
                false => v_edges.push(((a.0, b.1, a.1), (j, i))),
            }
        } else {
            match a.0 < b.0 {
                true  => h_edges.push(((a.1, a.0, b.0), (i, j))),
                false => h_edges.push(((a.1, b.0, a.0), (j, i))),
            }
        }
    }

    areas.iter()
        .rev()
        .find_map(|&(area, (i, j))| {
            let (a_x, a_y) = corners[i];
            let (b_x, b_y) = corners[j];

            let (a_x, b_x) = match a_x < b_x {
                true => (a_x, b_x),
                false => (b_x, a_x),
            };
            let (a_y, b_y) = match a_y < b_y {
                true => (a_y, b_y),
                false => (b_y, a_y),
            };

            for &((v_x, v_y_1, v_y_2), _) in v_edges.iter() {
                if a_x < v_x && v_x < b_x && (
                    (v_y_1 <=  a_y && a_y < v_y_2) ||
                    (v_y_1 < b_y && b_y <=  v_y_2)
                ) {
                    return None
                }
            }

            for &((v_y, v_x_1, v_x_2), _) in h_edges.iter() {
                if a_y < v_y && v_y < b_y && (
                    (v_x_1 <=  a_x && a_x <  v_x_2) ||
                    (v_x_1 < b_x && b_x <= v_x_2)
                ) {
                    return None
                }
            }

            Some(area)
        })
        .expect("No valid reactangle found")
}

