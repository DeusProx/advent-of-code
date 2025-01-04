use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/6/input").expect("Cannot read input file");
    let ways_to_beat_record = calc_ways_to_beat_the_record(&input);
    println!("ways to beat records: {}", ways_to_beat_record);
}

fn calc_ways_to_beat_the_record(input: &str) -> i64 {
    let data = input.lines()
        .map(|line| {
            let (_, data) = line.split_once(":").unwrap();
            let fixed_data = data.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<i64>().unwrap();
            vec![fixed_data]
        })
        .collect::<Vec<Vec<i64>>>();
    let [ time, distance ] = &*data else {
        panic!("Bad Input data")
    };
    time.iter()
        .zip(distance.iter())
        .map(|(time, distance)| pq(-*time, *distance))
        .map(|(start, end)| (end - 1.).ceil() as i64 - (start + 1.).floor().round() as i64 + 1)
        .product()
}

fn pq(p: i64, q: i64) -> (f64, f64) {
    let static_part = - (p as f64) / 2.;
    let dynamic_part = f64::sqrt(static_part.powi(2) - q as f64);
    (
        static_part - dynamic_part,
        static_part + dynamic_part,
    )
}

#[cfg(test)]
mod tests {
    use crate::calc_ways_to_beat_the_record;


    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/6/testinput").expect("Cannot read input file");
        let ways_to_beat_record = calc_ways_to_beat_the_record(&input);
        assert!(ways_to_beat_record == 71503);
    }
}

