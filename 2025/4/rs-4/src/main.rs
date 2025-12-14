// same puzzle but play around with rusts structs and methods

// 64 is occupied 46 is free
const ROLL:u8 =64;
const FREE:u8 =46;

struct Puzzle
{
    contents: String,
    result: usize,

    cols : usize,
    rows : usize,
    room : Vec<Vec<u8>>
}

impl Puzzle {


    fn new( filename: &str)->Puzzle
    {
        let contents :  String =std::fs::read_to_string(filename).expect("File not found");
        let cols = contents.lines().last().unwrap().len();
        let rows= contents.lines().count();
        // make the room larger to avoid stupid boundary cases when checking neighbour fields
        let mut room :Vec<Vec<u8>>=vec![vec![FREE; cols+2]; rows+2];
 
        let mut row=1;
        for line in contents.lines() {
            // insert "free" space where the walls are because they count as "free"
            room[row]=line.as_bytes().to_vec(); 
            room[row].push(FREE);
            room[row].insert(0,FREE);
            row+=1;      
        }
        Puzzle {
            contents,
            result : 0,
            cols,
            rows,
            room,
        }
    }


    fn count_surrounding_rolls(&self, col:usize, row:usize ) -> usize {
        let mut count=0;
        if self.room[row-1][col-1] == ROLL{ count+=1;}
        if self.room[row-0][col-1] == ROLL{ count+=1;}
        if self.room[row+1][col-1] == ROLL{ count+=1;}

        if self.room[row-1][col-0] == ROLL{ count+=1;}

        if self.room[row+1][col-0] == ROLL{ count+=1;}

        if self.room[row-1][col+1] == ROLL{ count+=1;}
        if self.room[row-0][col+1] == ROLL{ count+=1;}
        if self.room[row+1][col+1] == ROLL{ count+=1;}
    
        count
    }


    fn solve(& mut self) -> usize
    {
        loop {
            
            let mut rolls_to_remove:Vec<(usize,usize)>=vec![];
            let mut removed_count=0;

            for r in 1.. self.rows+1{
                for c in 1..self.cols+1{
                    if self.room[r][c]==ROLL{
                        let neighbours = self.count_surrounding_rolls(c,r);
                        if neighbours <4 {
                            removed_count+=1;
                            rolls_to_remove.push((r,c));
                        }
                    }         
                } 
            }
            for (r,c ) in rolls_to_remove{
                self.room[r][c]=FREE;
            }
            self.result += removed_count;
            //println!("removed {}",removed_count);
            if removed_count ==0
            {
                break;
            }
        }
        self.result

    }

}

use std::time::Instant;

fn main() {
    println!("AoC 2025 Riddle 4b");
    //let filename = "../../4/test.txt";
    let filename = "../../4/riddle.txt";
    let mut puzzle = Puzzle::new(filename);
    let start = Instant::now();
    puzzle.solve();
    let elapsed = start.elapsed();
    println!("Result: {}", puzzle.result);
    println!("Millis: {} ms", elapsed.as_millis());

}

/*
--- Part Two ---

Now, the Elves just need help accessing as much of the paper as they can.

Once a roll of paper can be accessed by a forklift, it can be removed. Once a roll of paper is removed, the forklifts might be able to access more rolls of paper, which they might also be able to remove. How many total rolls of paper could the Elves remove if they keep repeating this process?

Starting with the same example as above, here is one way you could remove as many rolls of paper as possible, using highlighted @ to indicate that a roll of paper is about to be removed, and using x to indicate that a roll of paper was just removed:

Initial state:
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.

Remove 13 rolls of paper:
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.

Remove 12 rolls of paper:
.......x..
.@@.x.x.@x
x@@@@...@@
x.@@@@..x.
.@.@@@@.x.
.x@@@@@@.x
.x.@.@.@@@
..@@@.@@@@
.x@@@@@@@.
....@@@...

Remove 7 rolls of paper:
..........
.x@.....x.
.@@@@...xx
..@@@@....
.x.@@@@...
..@@@@@@..
...@.@.@@x
..@@@.@@@@
..x@@@@@@.
....@@@...

Remove 5 rolls of paper:
..........
..x.......
.x@@@.....
..@@@@....
...@@@@...
..x@@@@@..
...@.@.@@.
..x@@.@@@x
...@@@@@@.
....@@@...

Remove 2 rolls of paper:
..........
..........
..x@@.....
..@@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@x.
....@@@...

Remove 1 roll of paper:
..........
..........
...@@.....
..x@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...

Remove 1 roll of paper:
..........
..........
...x@.....
...@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...

Remove 1 roll of paper:
..........
..........
....x.....
...@@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...

Remove 1 roll of paper:
..........
..........
..........
...x@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@...

Stop once no more rolls of paper are accessible by a forklift. In this example, a total of 43 rolls of paper can be removed.

Start with your original diagram. How many rolls of paper in total can be removed by the Elves and their forklifts?


*/