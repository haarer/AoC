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
    operators: Vec<OPERATOR>,
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

        let mut nums: Vec<Vec<usize>> = [].to_vec();
        // process lines except last as numbers
        for numlines in input_array.iter().take(input_array.len() - 1) {
            let nl: Vec<usize> = numlines
                .iter()
                .map(|&x| x.parse::<usize>().unwrap())
                .collect();
            nums.push(nl);
        }

        // lazy parse last line as ops
        let ops: Vec<OPERATOR> = input_array
            .iter()
            .next_back()
            .unwrap()
            .iter()
            .map(|&x| if x.contains('+') { PLUS } else { MAL })
            .collect();

        Puzzle {
            contents,
            result1: 0,
            result2: 0,
            numbers: nums,
            operators: ops,
        }
    }

    fn newVerticalTranspose(filename: &str) -> Puzzle {
        use crate::OPERATOR::*;

        let contents: String = std::fs::read_to_string(filename).expect("File not found");

        let mut input_array: Vec<Vec<char>> = [].to_vec();

        //put single char into array
        for line in contents.lines() {
            let chars_in_line = line.chars().collect();
            input_array.push(chars_in_line);
        }

        let rows = input_array.len() - 1;
        let cols = input_array[0].iter().len();
        let mut operands: Vec<usize> = vec![];

        let mut result=0;
        let mut last_op=' ';

        for col in (0..cols).rev() {
            let mut number = "".to_string();
            for row in 0..rows {
                let digit = input_array[row][col];
                number.push(digit);
            }

            //println!("num={}", number);
            let num: Result<usize, _> = number.trim().parse();
            let mut v;
            match num {
                Ok(num) => {
                    operands.push(num);
                }
                Err(_) => {
                    v = match last_op {
                        '+' => 0,
                        '*' => 1,
                        _ => 0,
                    };
                    for o in &operands {
                        //println!("op {}", o);
                        match last_op {
                            '+' => {
                                v += o;
                            }
                            '*' => {
                                v *= o;
                            }
                            _ => {}
                        }
                    }
                    operands=vec![];
                    result+= v;
                }
            }
            last_op = input_array[rows][col];
            //    if input_array[row][col]
            if col == 2
            {
                println!("achtung");
            }
        }

        Puzzle {
            contents,
            result1: 0,
            result2: result,
            numbers: vec![],
            operators: vec![],
        }
    }

    fn solve1(&mut self) {
        // prepare "accu" with neutral element per op
        let mut workbuffer: Vec<usize> = self
            .operators
            .iter()
            .map(|x| if *x == OPERATOR::MAL { 1 } else { 0 })
            .collect();

        // run operators "in parallel"
        for nums_per_task in 0..self.numbers.len() {
            for task_num in 0..self.numbers[0].len() {
                let n = self.numbers[nums_per_task][task_num];
                match self.operators[task_num] {
                    OPERATOR::MAL => workbuffer[task_num] *= n,
                    OPERATOR::PLUS => workbuffer[task_num] += n,
                }
            }
        }

        // sum up workbuffer

        self.result1 = workbuffer.iter().fold(0, |accu, x| accu + x);
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

    let mut puzzle2 = Puzzle::newVerticalTranspose(filename);
    let start = Instant::now();

    //puzzle.solve2();
    let elapsed = start.elapsed();
    println!("Result2: {}", puzzle2.result2);
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


--- Part Two ---

The big cephalopods come back to check on how things are going. When they see that your grand total doesn't match the one expected by the worksheet, they realize they forgot to explain how to read cephalopod math.

Cephalopod math is written right-to-left in columns. Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom. (Problems are still separated with a column consisting only of spaces, and the symbol at the bottom of the problem is still the operator to use.)

Here's the example worksheet again:

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +

Reading the problems right-to-left one column at a time, the problems are now quite different:

    The rightmost problem is 4 + 431 + 623 = 1058
    The second problem from the right is 175 * 581 * 32 = 3253600
    The third problem from the right is 8 + 248 + 369 = 625
    Finally, the leftmost problem is 356 * 24 * 1 = 8544

Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.

Solve the problems on the math worksheet again. What is the grand total found by adding together all of the answers to the individual problems?



 */
