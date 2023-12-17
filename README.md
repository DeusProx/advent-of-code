# Advent of Code

I decided I want to train a bit Rust with this years (2023) [advent of code](https://adventofcode.com/).
I don't know yet how far I get, but at least I started it.
You should try it, too!

## State

- [x] Day 1
- [x] Day 2
  - Code felt too expressive. Got some improvements from [reddit](https://www.reddit.com/r/rust/comments/189a5tu/string_manipulation_in_rust_advent_of_code/).
- [ ] Day 3
  - TODO: part 2.
- [x] Day 4
- [x] Day 5
  - Brute forced part 2 on a R9 5950x
  ```bash
      % time cargo run --release --bin p2
        Compiling day-05 v0.1.0 (/home/deusprox/git/deusprox/advent-of-code/day-05)
         Finished release [optimized] target(s) in 0.30s
          Running `target/release/p2`
    lowest point: 84206669
    cargo run --release --bin p2  100,33s user 6,95s system 100% cpu 1:47,09 total
  ```
  - TODO: improve part 2 by going the other way around: location -> seed from 0 to u64::MAX
- [x] Day 6
- [x] Day 7
- [x] Day 8
- [x] Day 9
- [ ] Day 10
- [ ] Day 11
- [ ] Day 12
- [ ] Day 13
- [ ] Day 14
- [x] Day 15
- [x] Day 16
  - Trying out parallelizing with rayon. See `day-16/src/part2.rayon`.
    - without rayon: ~3.3 seconds
    - with rayon:    ~0.2 seconds (on a R9 5950x)
- [ ] Day 17
- [ ] Day 18
- [ ] Day 19
- [ ] Day 20
- [ ] Day 21
- [ ] Day 22
- [ ] Day 23
- [ ] Day 24
- [ ] Day 25

