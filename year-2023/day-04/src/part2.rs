use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/4/input").expect("Cannot read input file");
    let output = count_scratchcard_copies(input);
    println!("scratchcard_copies_count: {}", output);
}

fn count_scratchcard_copies(input: String) -> usize {
    let card_table: Vec<usize> = input.trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let (_, data) = line.split_once(": ").unwrap();
            let (winning_numbers_data, picked_numbers_data) = data.split_once(" | ").unwrap();

            let winnings_numbers: Vec<i32> = winning_numbers_data.split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();
            let picked_numbers: Vec<i32> = picked_numbers_data.split_whitespace()
                .map(|n| n.parse().unwrap())
                .filter(|n| winnings_numbers.contains(n))
                .collect();

            picked_numbers.len()
        })
        .collect();

    let mut counter_table: Vec<usize> = std::iter::repeat(1).take(card_table.len()).collect();
    for i in 0..card_table.len() {
        let cards = card_table[i] + 1;
        for j in 1..cards {
            let x = i + j;
            if x > card_table.len() - 1 {
                break;
            }
            counter_table[x] += counter_table[i];
        }
    }

    counter_table.iter().sum()
}


#[cfg(test)]
mod tests {
    use crate::count_scratchcard_copies;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/4/testinput").expect("Cannot read input file");
        let scratchcard_copies_count = count_scratchcard_copies(input.to_string());
        assert!(scratchcard_copies_count == 30);
    }
}

