use std::fs;

fn main() {
    let input = fs::read_to_string("./input").expect("Cannot read input file")
        .parse().expect("Cannot parse input file");
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

    const INPUT: &str = r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;

    #[test]
    fn test() {
        let scratchcard_copies_count = count_scratchcard_copies(INPUT.to_string());
        assert!(scratchcard_copies_count == 30);
    }
}

