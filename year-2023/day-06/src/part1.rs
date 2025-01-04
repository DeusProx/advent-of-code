fn main() {
    let input: String = std::fs::read_to_string("../../data/2023/day/6/input").expect("Cannot read input file");
    let ways_to_beat_record = calc_ways_to_beat_the_record(&input);
    println!("ways to beat records: {}", ways_to_beat_record);
}

fn calc_ways_to_beat_the_record(input: &str) -> i32 {
    let data = input.lines()
        .map(|line| {
            let (_, data) = line.split_once(":").unwrap();
            let deduped_data = dedup_ws(data);
            deduped_data.split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let [ time, distance ] = &*data else {
        panic!("Bad Input data")
    };
    time.iter()
        .zip(distance.iter())
        .map(|(time, distance)| pq(-*time, *distance))
        .map(|(start, end)| (end - 1.).ceil() as i32 - (start + 1.).floor().round() as i32 + 1)
        .product()
}

fn pq(p: i32, q: i32) -> (f32, f32) {
    let static_part = - (p as f32) / 2.;
    let dynamic_part = f32::sqrt(static_part.powi(2) - q as f32);
    (
        static_part - dynamic_part,
        static_part + dynamic_part,
    )
}

fn dedup_ws(input: &str) -> String {
    let mut was_ws = false;
    input.trim_start().chars().filter_map(|c| {
        let is_ws = c == ' ';
        let is_trimmed = match was_ws && is_ws {
            true => None,
            false => Some(c),
        };
        was_ws = is_ws;
        is_trimmed
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::calc_ways_to_beat_the_record;


    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/6/testinput").expect("Cannot read input file");
        let ways_to_beat_record = calc_ways_to_beat_the_record(&input);
        assert!(ways_to_beat_record == 288);
    }
}

