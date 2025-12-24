#[derive(PartialEq, Eq)]
enum OPERATOR {
    MAL,
    PLUS,
}

struct Puzzle {
    contents: String,
    result1: usize,
    result2: usize,
    numbers: Vec<Vec<usize>>,
    operators:Vec<OPERATOR>,
}

impl Puzzle {
    
        fn new(filename: &str) -> Puzzle {
                use crate::OPERATOR::*;

            let contents: String = std::fs::read_to_string(filename).expect("File not found");

        let mut input_array: Vec<Vec<&str>> = [].to_vec();

        // read split elements from lines as strings
        for line in contents.lines() {
            let numbers_in_line = line.split_ascii_whitespace().collect();
            input_array.push(numbers_in_line);
        }

          let mut nums: Vec<Vec<usize>>=[].to_vec();
        // process lines except last as numbers
        for numlines in input_array.iter().take(input_array.len()-1){
            let nl : Vec<usize> = numlines.iter().map(|&x|x.parse::<usize>().unwrap()).collect();
            nums.push(nl);
        }

        // lazy parse last line as ops
        let ops : Vec<OPERATOR> =input_array.iter().next_back().unwrap().iter().map(|&x|if x.contains('+') {PLUS} else{MAL}).collect();



        Puzzle {
            contents,
            result1: 0,
            result2: 0,
            numbers: nums,
            operators: ops,
        }
    }

    fn solve1(&mut self) {

        // prepare "accu" with neutral element per op
        let mut workbuffer:Vec<usize>=self.operators.iter().map(|x|if *x==OPERATOR::MAL {1} else {0}).collect();

        // run operators "in parallel"
        for nums_per_task in 0..self.numbers.len()
        {
            for task_num in 0..self.numbers[0].len()
            {
                let n=self.numbers[nums_per_task][task_num];
                match self.operators[task_num]{
                    OPERATOR::MAL => workbuffer[task_num]*=n,
                    OPERATOR::PLUS => workbuffer[task_num]+=n,
                }
            }
        }

        // sum up workbuffer

        self.result1=workbuffer.iter().fold(0,|accu,x|accu + x);

    }

    fn solve2(&mut self) {
    }
}
use core::num;
use std::{arch::x86_64, path::MAIN_SEPARATOR, time::Instant, usize};

fn main() {

    println!("AoC 2025 Riddle 6");
    //let filename = "../6/test.txt";
    let filename = "../6/riddle.txt";
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
 
--- Day 6: Trash Compactor ---

After helping the Elves in the kitchen, you were taking a break and helping them re-enact a movie scene when you over-enthusiastically jumped into the garbage chute!

A brief fall later, you find yourself in a garbage smasher. Unfortunately, the door's been magnetically sealed.

As you try to find a way out, you are approached by a family of cephalopods! They're pretty sure they can get the door open, but it will take some time. While you wait, they're curious if you can help the youngest cephalopod with her math homework.

Cephalopod math doesn't look that different from normal math. The math worksheet (your puzzle input) consists of a list of problems; each problem has a group of numbers that need to be either added (+) or multiplied (*) together.

However, the problems are arranged a little strangely; they seem to be presented next to each other in a very long horizontal list. For example:

123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  

Each problem's numbers are arranged vertically; at the bottom of the problem is the symbol for the operation that needs to be performed. Problems are separated by a full column of only spaces. The left/right alignment of numbers within each problem can be ignored.

So, this worksheet contains four problems:

    123 * 45 * 6 = 33210
    328 + 64 + 98 = 490
    51 * 387 * 215 = 4243455
    64 + 23 + 314 = 401

To check their work, cephalopod students are given the grand total of adding together all of the answers to the individual problems. In this worksheet, the grand total is 33210 + 490 + 4243455 + 401 = 4277556.

Of course, the actual worksheet is much wider. You'll need to make sure to unroll it completely so that you can read the problems clearly.

Solve the problems on the math worksheet. What is the grand total found by adding together all of the answers to the individual problems?
 */
