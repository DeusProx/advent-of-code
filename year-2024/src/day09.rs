use std::{cmp::Ordering, iter, time::Instant};


#[aoc_macro::bench()]
pub fn day9_part1() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/9/input").expect("Cannot read input");

    let mut blocks = Block::parse(&input);

    let mut position: usize = 0;
    let mut left = 0;
    let mut right = blocks.len() - 1;
    let mut counter: u64 = 0;

    while left <= right {
        match blocks[left] {
            Block::File(size, id) => {
                counter += (position..position + size)
                    .zip(iter::repeat_n(id, size))
                    .map(|(position, id)| position as u64 * id as u64)
                    .sum::<u64>();
                position += size;
            },
            Block::Free(mut size) => {
                while size > 0  {
                    while matches!(blocks[right], Block::Free(_)) {
                        right -= 1;
                    }
                    if left > right {
                        break
                    }

                    let block = &blocks[right];
                    let id = block.id().expect("block to have an id");
                    let copy_size = block.size().min(size);
                    counter += (position..position + copy_size)
                        .zip(iter::repeat_n(id, copy_size))
                        .map(|(position, id)| position as u64 * id)
                        .sum::<u64>();
                    position += copy_size;

                    match block.size().cmp(&size) {
                        Ordering::Less => {
                            blocks[right] = Block::Free(0);
                            size -= copy_size;
                        },
                        Ordering::Equal => {
                            blocks[right] = Block::Free(0);
                            size = 0;
                        },
                        Ordering::Greater => {
                            blocks[right] = Block::File(block.size() - copy_size, id);
                            size = 0
                        },

                    }

                }
            }
        };
        left += 1;
    }

    counter
}

#[derive(Debug, Clone)]
pub enum Block {
    // size, id
    File(usize, u64),
    // size
    Free(usize),
}
impl Block {
    pub fn parse(input: &str) -> Vec<Self> {
        input.char_indices()
            .take(input.len() - 1) // Cut off DLE char
            .map(|(index, c)| {
                let id: u64 = index as u64;
                let size = (c as u8 - 48u8) as usize;
                match id % 2 {
                    1 => Block::Free(size),
                    0 => Block::File(size, id / 2),
                    _ => panic!("in the disco"),
                }
            })
            .collect::<Vec<Block>>()
    }
    pub fn size(&self) -> usize {
        match self {
            Self::File(size, _) => *size,
            Self::Free(size) => *size,
        }
    }
    pub fn id(&self) -> Option<u64> {
        match self {
            Self::File(_, id) => Some(*id),
            Self::Free(_) => None,
        }
    }
}

// damn this code is ugly
// just wanted to get done at this point
#[aoc_macro::bench()]
pub fn day9_part2() -> u64 {
    // TODO: macro
    let input = std::fs::read_to_string("../data/2024/day/9/input").expect("Cannot read input");
    let mut files: Vec<(usize, u64)> = Vec::with_capacity(input.len());
    let mut spaces: Vec<(usize, Vec<(usize, u64)>)> = Vec::with_capacity(input.len() / 2);

    input.char_indices()
            .take(input.len() - 1) // Cut off DLE char
            .for_each(|(index, c)| {
                let id = index / 2;
                let size = (c as u8 - 48u8) as usize;
                match index % 2 {
                    1 => spaces.push((size, Vec::with_capacity(10))),
                    0 => files.push((size, id as u64)),
                    _ => panic!("in the disco"),
                };
            });

    for file in files.iter_mut().rev() {
        for (space_id, space) in spaces.iter_mut().enumerate() {
            if space_id >= file.1 as usize {
                continue
            }
            let free = space.0 - space.1.iter().map(|(size, _)| size).sum::<usize>();

            match file.0 <= free {
                true => {
                    space.1.push(file.clone());
                    file.1 = 0;
                    break;
                },
                false => continue,
            }
        }
    }

    let mut position = 0;
    let mut counter = 0;

    for index in 0..input.len() / 2 - 1 {
        let (size, id) = files[index];
        counter += (position..position + size)
            .zip(iter::repeat_n(id, size))
            .map(|(position, id)| position as u64 * id as u64)
            .sum::<u64>();
        position += size;

        let space = &spaces[index];
        space.1.iter().for_each(|(size, id)| {

            counter += (position..position + size)
                .zip(iter::repeat_n(id, *size))
                .map(|(position, id)| position as u64 * *id as u64)
                .sum::<u64>();
            position += size;
        });

        let free = space.0 - space.1.iter().map(|(size, _)| size).sum::<usize>();
        position += free;
    }

    counter
}

