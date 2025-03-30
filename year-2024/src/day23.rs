use std::{char, collections::{HashMap, HashSet}, hash::Hash, time::Instant};
use rayon::prelude::*;

#[aoc_macro::bench()]
pub fn part1() -> usize {
    let input = std::fs::read_to_string("../data/2024/day/23/input").expect("Cannot read input");
    let rules = parse(&input);

    let rules_iter = rules.par_iter();
    let groups: Vec<(Vec<u16>, Vec<u16>, Vec<u16>)> = rules_iter.clone()
        .enumerate()
        .flat_map(|(i1, rule1)| {
            let r = rules_iter.clone().skip(i1 + 1);
            r.clone()
                .enumerate()
                .filter(|(_, rule2)| rule2.iter().any(|id| rule1.contains(id)))
                .filter_map(move |(i2, rule2)| {
                    let r = r.clone().skip(i2 + 1);
                    r.find_any(|rule3| rule3.iter().all(|id| rule1.contains(id) || rule2.contains(id)))
                        .map(|rule3| (rule1.clone(), rule2.clone(), rule3.clone()))
                })
        })
        .collect();

    groups
        .iter()
        .filter(|(a, b, c)|
            a.iter()
                .chain(b.iter())
                .chain(c.iter())
                .any(|id| (id >> 8) as u8 == 't' as u8)
        )
        .count()

}

#[aoc_macro::bench()]
pub fn part2() -> String {
    let input = std::fs::read_to_string("../data/2024/day/23/input").expect("Cannot read input");

    let (vertices, edges) = input.lines().fold(
        (HashSet::new(), HashMap::new()),
        |(mut vertices, mut edges), line| {
            let (left, right) = line.split_once("-").unwrap();

            vertices.insert(left);
            vertices.insert(right);

            edges.entry(left).or_insert_with(HashSet::new).insert(right);
            edges.entry(right).or_insert_with(HashSet::new).insert(left);

            (vertices, edges)
        }
    );

    let largest_clique = bron_kerbosch(
        &HashSet::new(),
        &mut vertices.iter().cloned().collect(),
        &mut HashSet::new(),
        &edges,
    );
    let mut largest_clique: Vec<&str> = largest_clique.into_iter().collect();
    largest_clique.sort();

    largest_clique.join(",")
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
// Goal: Finds the maximal cliques (fully connected subgraph) in an undirected graph
fn bron_kerbosch<'a, T: Eq + Hash + ?Sized>(
    current_clique: &HashSet<&'a T>,        // "R"-Set: vertices in current clique
    potential_members: &mut HashSet<&'a T>, // "P"-Set: vertices that can be added
    outsiders: &mut HashSet<&'a T>,         // "X"-Set: vertices that cannot be added
    edges: &HashMap<&'a T, HashSet<&'a T>>
) -> HashSet<&'a T> {
    if potential_members.is_empty() && outsiders.is_empty() {
        return current_clique.clone();
    }

    let mut largest_clique = HashSet::new();
    for vertex in potential_members.clone() {
        let mut current_clique = current_clique.clone();
        current_clique.insert(vertex);

        let neighbors = edges.get(vertex).unwrap();
        let candidate_largest_clique = bron_kerbosch(
            &current_clique,
            &mut potential_members.intersection(neighbors).cloned().collect(),
            &mut outsiders.intersection(neighbors).cloned().collect(),
            edges
        );

        if candidate_largest_clique.len() > largest_clique.len() {
            largest_clique = candidate_largest_clique;
        }

        potential_members.remove(vertex);
        outsiders.insert(vertex);
    }

    largest_clique.into_iter().collect()
}

fn parse(input: &str) -> Vec<Vec<u16>> {
    let mut rules = input.lines()
        .map(|line| {
            let mut rule = line.split("-")
                .map(encode)
                .collect::<Vec<u16>>();
            rule.sort();
            rule
        })
        .collect::<Vec<Vec<u16>>>();
    rules.sort();

    rules
}

pub fn encode(name: &str) -> u16 {
    let numbers: Vec<u16> = name.chars()
        .map(|n| n as u8 as u16)
        .collect();

    numbers[0] << 8 ^ numbers[1]
}
#[test]
fn encoding_works() {
    assert_eq!(encode("aa"), 24929);
}

pub fn decode(value: u16) -> String {
    [
        (value >> 8) as u8 as char,
        value as u8 as char
    ].into_iter().collect()
}
#[test]
fn decoding_works() {
    assert_eq!(decode(24929), "aa");
}

