use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> usize {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/8/input").expect("Cannot read input");

    let nodes: Vec<Vec<f64>> = input.lines()
        .map(|line| line.split(',').map(|n| n.parse::<f64>().unwrap()).collect())
        .collect();

    let mut distances: Vec<(f64, (usize, usize))> = Vec::with_capacity(nodes.len() * (nodes.len() - 1) / 2);

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let euclidean_distance = (
                (nodes[i][0] - nodes[j][0]).powi(2) +
                (nodes[i][1] - nodes[j][1]).powi(2) +
                (nodes[i][2] - nodes[j][2]).powi(2)
            ).sqrt();
            distances.push((euclidean_distance, (i, j)));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&(b.0)).unwrap());
    let choosen_edges = distances.iter()
        .take(1000) // 10 for test
        .map(|(_, edge)| edge);

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for &(i, j) in choosen_edges {
        let mut hits = Vec::new();
        for k in 0..circuits.len() {
            if circuits[k].contains(&i) || circuits[k].contains(&j) {
                hits.push(k);
            }
        }

        match hits.len() {
            0 => circuits.push(vec![i, j]),
            1 => {
                let circuit = circuits.get_mut(hits[0]).unwrap();
                if !circuit.contains(&i) { circuit.push(i) }
                if !circuit.contains(&j) { circuit.push(j) }
            },
            2 => {
                let mut to_copy = circuits.swap_remove(hits[1]);
                circuits[hits[0]].append(&mut to_copy);
                circuits[hits[0]].dedup();
            },
            _ => panic!("in the disco"),
        }
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits.iter()
        .take(3)
        .map(|circuit| circuit.len())
        .product()
}

#[aoc_macro::bench()]
pub fn part2() -> f64 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/8/input").expect("Cannot read input");

    let nodes: Vec<Vec<f64>> = input.lines()
        .map(|line| line.split(',').map(|n| n.parse::<f64>().unwrap()).collect())
        .collect();

    let mut distances: Vec<(f64, (usize, usize))> = Vec::with_capacity(nodes.len() * (nodes.len() - 1) / 2);

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let euclidean_distance = (
                (nodes[i][0] - nodes[j][0]).powi(2) +
                (nodes[i][1] - nodes[j][1]).powi(2) +
                (nodes[i][2] - nodes[j][2]).powi(2)
            ).sqrt();
            distances.push((euclidean_distance, (i, j)));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&(b.0)).unwrap());

    let mut last_pair: (usize, usize) = (usize::MAX, usize::MAX);
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for &(i, j) in distances.iter().map(|(_, p)| p) {
        if circuits.len() == 1 && circuits[0].len() == nodes.len() {
            break;
        }
        last_pair = (i, j);
        let mut hits = Vec::new();
        for k in 0..circuits.len() {
            if circuits[k].contains(&i) || circuits[k].contains(&j) {
                hits.push(k);
            }
        }

        match hits.len() {
            0 => circuits.push(vec![i, j]),
            1 => {
                let circuit = circuits.get_mut(hits[0]).unwrap();
                if !circuit.contains(&i) { circuit.push(i) }
                if !circuit.contains(&j) { circuit.push(j) }
            },
            2 => {
                let mut to_copy = circuits.swap_remove(hits[1]);
                circuits[hits[0]].append(&mut to_copy);
                circuits[hits[0]].dedup();
            },
            _ => panic!("in the disco"),
        }
    }

    nodes[last_pair.0][0] * nodes[last_pair.1][0]
}

