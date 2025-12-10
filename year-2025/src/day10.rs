use std::time::Instant;

#[aoc_macro::bench()]
pub fn part1() -> u32 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/10/input").expect("Cannot read input");

    input.lines()
        // parsing
        .map(|line| {
            let mut iter = line.split_whitespace();

            // joltages are not needed for this part
            _ = iter.next_back().unwrap();

            // indicator light diagram; e.g. [.##.]; encoded as u16
            let lights = iter.next().unwrap();
            let n = lights.len() - 2;
            let lights: u16 = lights[1..(lights.len() - 1)]
                .char_indices()
                .map(|(i, c)| match c {
                    '.' => 0,
                    '#' => 1 << i,
                    _ => panic!("in the disco"),
                })
                .sum();
            // println!("Lights: {lights:0n$b}");

            // the indexes/digits of the light a button press changes; encoded as u16
            let buttons: Vec<u16> = iter.map(|ns| {
                    ns[1..(ns.len() - 1)]
                        .split(',')
                        .map(|c| 1 << c.parse::<u16>().unwrap())
                        .sum()
                })
                // .inspect(|button| println!("Buttons: {button:0n$b}"))
                .collect();

            (n, lights, buttons)
        })
        // calculating button pushes
        .map(|(_n, lights, buttons)| {
            // println!();
            hamming_weight_iter(buttons.len() as u16)
                .find(|attempt| {
                    let value = buttons.iter()
                        .enumerate()
                        .map(|(index, &button)| {
                            let nth_bit = (attempt >> index) & 1_u16;
                            match nth_bit {
                                0 => 0,
                                1 => button,
                                _ => panic!("in the disco"),
                            }
                        })
                        .fold(0, |acc, v| acc ^ v);
                    // println!("Attempt: {attempt:0_n$b}: {lights:0_n$b} == {value:0_n$b} => {}", lights == value);
                    lights == value
                }).unwrap()
                .count_ones()
        })
        .sum()
}

/**
* Given an input of n digits
* Gives back an iterator for hamming-weight ordered numbers with max n digits
*
* E.g. n = 3 => [ 000, 001, 010, 100, 011, 101, 110, 111 ]
*/
fn hamming_weight_iter<T>(n: T) -> impl Iterator<Item = T>
where
    T: Copy + Into<u64> + TryFrom<u64> // for a more generic version one should u128,
                                       // but this shaves of a few more microseconds
{
    let n: u64 = n.into();
    let bit_width = 8 * core::mem::size_of::<T>();
    assert!(n as usize <= bit_width, "n={n} exceeds bit width of T ({bit_width})");

    let limit = 1u64 << n;
    let mut k = 0u64; // current hamming weight
    let mut c = 0u64; // current combination

    std::iter::from_fn(move || {
        if k == 0 {
            k = 1;
            return T::try_from(0).ok();
        }

        loop {
            if k > n {
                return None;
            }
            if c == 0 {
                c = (1u64 << k) - 1;
            }
            if c >= limit {
                k += 1;
                c = 0;
                continue;
            }


            let out = c;

            // "gosper step" to generate next k-combination
            let u = c & c.wrapping_neg();
            let v = c.wrapping_add(u);
            c = v | (((v ^ out) / u) >> 2);
            // c = v | ((v ^ out) >> (u.trailing_zeros() + 2));

            return T::try_from(out).ok();
        }
    })
}


#[aoc_macro::bench()]
pub fn part2() -> u32 {
    // TODO: Solve with macro
    let input = std::fs::read_to_string("../data/2025/day/10/test").expect("Cannot read input");
    // let input = std::fs::read_to_string("../data/2025/day/10/input").expect("Cannot read input");

    input.lines()
        // parsing
        .map(|line| {
            let mut iter = line.split_whitespace();

            // indicator light diagrams are not needed for this part
            _ = iter.next().unwrap();

            let mut iter = iter.map(|ns| {
                ns[1..(ns.len() - 1)]
                    .split(',')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            });

            let joltages: Vec<u32>     = iter.next_back().unwrap();
            let buttons: Vec<Vec<u32>> = iter.map(|set| {
                let mut arr = vec![0; joltages.len()];
                for i in set {
                    arr[i as usize] = 1;
                }
                arr
            }).collect();

            (joltages, buttons)
        })
        // calculating button pushes
        .map(|(_joltages, _buttons)| {
            // TODO: Gaussian Elimination????
            0
        })
        .sum()
}

