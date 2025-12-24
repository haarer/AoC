struct Puzzle {
    contents: String,
    result1: usize,
    result2: usize,

    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}
enum Mode {
    RANGES,
    IDS,
}

#[derive(PartialEq, Eq)]
enum OverlapKind {
    R2OverlapsR1Left,
    R2OverlapsR1Right,
    R2WithinR1,
    R1WithinR2,
    NoOverlap,
}

fn check_ranges_overlap(r1: &(usize, usize), r2: &(usize, usize)) -> OverlapKind {
    // test for overlap

    // is second ranges first element within first range ?
    if r2.0 <= r1.1 && r2.0 >= r1.0 {
        // is second ranges last element also within first range ?
        if r2.1 <= r1.1 && r2.1 >= r1.0 {
            //-> r2 fully falls in r1 -> drop r2
            return OverlapKind::R2WithinR1;
        } else {
            //-> r2 overlaps r1 right -> extend r1 to end of r2 and drop r2
            return OverlapKind::R2OverlapsR1Right;
        }
    } else {
        // second ranges first element is not in first range
        // test for other overlap cases
        // is second ranges last element within first range ?
        if r2.1 <= r1.1 && r2.1 >= r1.0 {
            //-> r2 overlaps r1 left -> extend r2 to end of r1 and drop r1
            return OverlapKind::R2OverlapsR1Left;
        } else {
            //second ranges last element is also not in first range
            // is r2 after r1 or before r1 ?
            if r2.0 > r1.1 || r2.1 < r1.0 {
                return OverlapKind::NoOverlap;
            } else {
                // this leaves only this
                return OverlapKind::R1WithinR2;
            }
        }
    }
}

impl Puzzle {
    fn new(filename: &str) -> Puzzle {
        use crate::Mode::*;
        let contents: String = std::fs::read_to_string(filename).expect("File not found");

        let mut ranges: Vec<(usize, usize)> = [].to_vec();
        let mut ids: Vec<usize> = [].to_vec();
        let mut mode = RANGES;

        for line in contents.lines() {
            match mode {
                RANGES => {
                    if line != "" {
                        let (start, end) = line.split_once("-").unwrap();
                        ranges.push((
                            start.parse().expect("cant parse start"),
                            end.parse().expect("cant parse end"),
                        ));
                    } else {
                        mode = IDS;
                    }
                }
                IDS => {
                    ids.push(line.parse().expect("cant parse id"));
                }
            }
        }
        println!("parsed {} ranges", ranges.iter().count());
        println!("parsed {} ids", ids.iter().count());
        Puzzle {
            contents,
            result1: 0,
            result2: 0,
            ranges,
            ids,
        }
    }

    fn solve1(&mut self) {
        let mut found_id_count = 0;
        for id in &self.ids {
            for (start, end) in &self.ranges {
                if id >= start && id <= end {
                    found_id_count += 1;
                    break;
                }
            }
        }

        self.result1 = found_id_count;
    }

    // strategy: merge overlapping range fragments, then sum up range sizes, there are 186 ranges -> O(n²/2) is acceptable
    fn solve2(&mut self) {
        // obtain a copy for inplace mod
        let mut new_ranges: Vec<(usize, usize)> = self.ranges.clone();
        let mut iterations = 0;
        let mut lastlen = 0;

        // run until length of ranges no longer changes
        loop {
            //println!("iteration {}",iterations);

            // compare all ranges..
            'outer: for r1index in 0..new_ranges.len() - 1 {
                let r1 = new_ranges[r1index];
                //println!("testing range {}-{}",r1.0,r1.1);
                // .. against the others
                for r2index in r1index + 1..new_ranges.len() {
                    let r2 = new_ranges[r2index];
                    //println!("against {}-{}",r2.0,r2.1);

                    // test for overlap
                    match check_ranges_overlap(&r1, &r2) {
                        OverlapKind::NoOverlap => {
                            // no action
                        }
                        OverlapKind::R1WithinR2 => {
                            // drop r1 because it is within a larger range r2
                            new_ranges.remove(r1index);
                            //println!("removed at {}",r2index);
                            break 'outer;
                        }
                        OverlapKind::R2WithinR1 => {
                            // drop r2 because it is within a larger range r1
                            new_ranges.remove(r2index);
                            //println!("removed at {}",r2index);
                            break 'outer;
                        }
                        OverlapKind::R2OverlapsR1Left => {
                            // merge r2 into r1, that is drop r2 and set r1 left boundary to begin of r2
                            new_ranges[r1index].0 = new_ranges[r2index].0;
                            new_ranges.remove(r2index);
                            //println!("removed at {}",r2index);
                            break 'outer;
                        }
                        OverlapKind::R2OverlapsR1Right => {
                            // merge r2 into r1, that is drop r2 and set r1 right boundary to end of r2
                            new_ranges[r1index].1 = new_ranges[r2index].1;
                            new_ranges.remove(r2index);
                            //println!("removed at {}",r2index);
                            break 'outer;
                        }
                    };
                }
            }

            if new_ranges.len() == lastlen {
                break;
            }
            lastlen = new_ranges.len();
            iterations += 1;
        }
        self.ranges = new_ranges;

        let mut num_of_ids = 0;
        for r in &self.ranges {
            num_of_ids += r.1 - r.0 + 1;
        }
        self.result2 = num_of_ids;
    }
}
use std::{time::Instant, usize};

fn main() {
    assert!(
        check_ranges_overlap(&(10, 20), &(30, 40)) == OverlapKind::NoOverlap,
        "test 1  for NoOverlap failed"
    );
    assert!(
        check_ranges_overlap(&(10, 20), &(15, 40)) == OverlapKind::R2OverlapsR1Right,
        "test 2 for R2OverlapsR1Right failed"
    );
    assert!(
        check_ranges_overlap(&(10, 20), &(5, 14)) == OverlapKind::R2OverlapsR1Left,
        "test 3  for R2OverlapsR1Left failed"
    );
    assert!(
        check_ranges_overlap(&(10, 20), &(12, 17)) == OverlapKind::R2WithinR1,
        "test 4 for R2WithinR1 failed"
    );
    assert!(
        check_ranges_overlap(&(10, 20), &(5, 22)) == OverlapKind::R1WithinR2,
        "test 5 for R1WithinR2 failed"
    );
    assert!(
        check_ranges_overlap(&(10, 20), &(21, 25)) == OverlapKind::NoOverlap,
        "test 6 for NoOverlap failed"
    );
    assert!(
        check_ranges_overlap(&(10, 20), &(8, 9)) == OverlapKind::NoOverlap,
        "test 7 for NoOverlap failed"
    );

    println!("AoC 2025 Riddle 5");
    //let filename = "../../5/test.txt";
    let filename = "../../5/riddle.txt";
    let mut puzzle = Puzzle::new(filename);

    let start = Instant::now();
    puzzle.solve1();
    let elapsed = start.elapsed();
    println!("Result1: {}", puzzle.result1);
    println!("Millis: {} ms", elapsed.as_millis());

    let start = Instant::now();
    puzzle.solve2();
    let elapsed = start.elapsed();
    println!("Result2: {}", puzzle.result2);
    println!("Millis: {} ms", elapsed.as_millis());
}

/*
--- Day 5: Cafeteria ---

As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.

You can hear a commotion coming from the kitchen. "At this rate, we won't have any time left to put the wreaths up in the dining hall!" Resolute in your quest, you investigate.

"If only we hadn't switched to the new inventory management system right before Christmas!" another Elf exclaims. You ask what's going on.

The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).

The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:

3-5
10-14
16-20
12-18

1
5
8
11
17
32

The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.

The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:

    Ingredient ID 1 is spoiled because it does not fall into any range.
    Ingredient ID 5 is fresh because it falls into range 3-5.
    Ingredient ID 8 is spoiled.
    Ingredient ID 11 is fresh because it falls into range 10-14.
    Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
    Ingredient ID 32 is spoiled.

So, in this example, 3 of the available ingredient IDs are fresh.

Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?


--- Part Two ---

The Elves start bringing their spoiled inventory to the trash chute at the back of the kitchen.

So that they can stop bugging you when they get new inventory, the Elves would like to know all of the IDs that the fresh ingredient ID ranges consider to be fresh. An ingredient ID is still considered fresh if it is in any range.

Now, the second section of the database (the available ingredient IDs) is irrelevant. Here are the fresh ingredient ID ranges from the above example:

3-5
10-14
16-20
12-18

The ingredient IDs that these ranges consider to be fresh are 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20. So, in this example, the fresh ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.

Process the database file again. How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?



*/
