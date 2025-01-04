use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input").expect("Cannot read input file");
    let result = calc(input);
    println!("Result: {}", result);
}


fn calc(input: String) -> u32 {
    let mut boxes: [Vec<(&str, u32)>; 256] = vec![Vec::new(); 256].try_into().expect("Noooooooooo");
    for seq in input.trim().split(",") {

        if seq.ends_with('-') {
            let label = seq.strip_suffix('-').unwrap();
            let boxx = boxes.get_mut(hash(label)).unwrap();
            let index = boxx.iter()
                .position(|(boxx_label, _)| &label == boxx_label);
            if let Some(i) = index {
                boxx.remove(i);
            }
            continue;
        }

        let (label, focal_length) = seq.split_once("=").unwrap();
        let boxx = boxes.get_mut(hash(label)).unwrap();
        let lens = (label, focal_length.parse::<u32>().unwrap());
        if let Some(el) = boxx.iter_mut().find(|(boxx_label, _)| &label == boxx_label){
            let _ = std::mem::replace(el, lens);
        } else {
            boxx.push(lens);
        }

    }

    boxes.iter().enumerate()
        .map(|(box_index, boxx)| {
            boxx.iter().enumerate()
                .map(move |(lens_index, (_, focal_length))| (box_index as u32 + 1) * (lens_index as u32 + 1) * focal_length)
        })
        .flatten()
        .sum()
}

fn hash(input: &str) -> usize {
    input.chars()
        .fold(0, |mut acc, c| {
            acc += c as usize;
            acc *= 17;
            acc %= 256;
            acc
        })
}

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn test() {
        let input: String = std::fs::read_to_string("./test").expect("Cannot read input file");
        let result = calc(input);
        assert!(result == 145);
    }
}

