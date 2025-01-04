use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../data/2023/day/5/input").expect("Cannot read input file");
    let almanac = Almanac::parse(&input);
    println!("lowest point: {}", almanac.get_lowest_location());
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<AlmanacMapRule>>,
}
impl Almanac {
    fn parse(input: &str) -> Almanac {
        let mut iter = input.split("\n\n");
        let seeds: Vec<u64> = iter.next().unwrap()
            .split_once(": ").unwrap()
            .1
            .split(" ")
            .map(|n| {
                n.parse::<u64>().unwrap()
            })
            .collect();
        let maps = iter.map(|data_map| {
            data_map.split_once(":\n").unwrap()
                .1.lines()
                .map(|line| AlmanacMapRule::parse(line))
                .collect()
        }).collect();
        Almanac {
            seeds,
            maps,
        }
    }
    fn convert_seeds(&self) -> Vec<u64> {
        self.seeds.iter().map(|seed| {
            let mut value = *seed;
            for map in self.maps.iter() {
                for rule in map {
                    match rule.convert(value) {
                        Some(converted) => {
                            value = converted;
                            break;
                        },
                        None => (),
                    }
                }
            }
            value
        })
        .collect::<Vec<u64>>()
    }
    fn get_lowest_location(&self) -> u64 {
        *self.convert_seeds().iter().min().unwrap()
    }
}

struct AlmanacMapRule {
    destination_start: u64,
    source_start: u64,
    length: u64
}
impl AlmanacMapRule {
    fn parse(line: &str) -> AlmanacMapRule {
        let mut iter = line.split(" ").map(|n| n.parse::<u64>().expect(&format!("Cannot parse {} to u64", n)));
        AlmanacMapRule {
            destination_start: iter.next().expect("destination_range_start cannot be read"),
            source_start: iter.next().expect("source_range_start cannot be read"),
            length: iter.next().expect("range_length cannot be read"),
        }
    }
    fn convert(&self, input: u64) -> Option<u64> {
        match self.source_start <= input && input < self.source_start + self.length {
            true => Some(input - self.source_start + self.destination_start),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Almanac;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("../../data/2023/day/5/testinput").expect("Cannot read input file");
        let almanac = Almanac::parse(&input);
        assert!(almanac.get_lowest_location() == 35);
    }
}

