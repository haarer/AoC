
const ROLL:u8 =64;
const FREE:u8 =46;

fn count_surrounding_rolls(col:usize,row:usize, v:&Vec<Vec<u8>>)->usize{
    let mut count=0;
    if v[row-1][col-1] == ROLL{ count+=1;}
    if v[row-0][col-1] == ROLL{ count+=1;}
    if v[row+1][col-1] == ROLL{ count+=1;}

    if v[row-1][col-0] == ROLL{ count+=1;}

    if v[row+1][col-0] == ROLL{ count+=1;}

    if v[row-1][col+1] == ROLL{ count+=1;}
    if v[row-0][col+1] == ROLL{ count+=1;}
    if v[row+1][col+1] == ROLL{ count+=1;}
  
    count
}


fn solve(filename: &str)-> usize
{
    let contents = std::fs::read_to_string(filename).expect("File not found");
    let mut result:usize= 0;

    let cols=contents.lines().last().unwrap().len();
    let rows =contents.lines().count();
    let mut room= vec![vec![FREE; cols+2];  rows+2];
    let mut row=1;
    for line in contents.lines() {
        room[row]=line.as_bytes().to_vec(); // 64 is occupied 46 is free
        room[row].push(FREE);
        room[row].insert(0,FREE);
        row+=1;      
    }
    loop {
        
        let mut rolls_to_remove:Vec<(usize,usize)>=vec![];
        let mut removed_count=0;

        for r in 1.. rows+1{
            for c in 1..cols+1{
                if room[r][c]==ROLL{
                    let neighbours = count_surrounding_rolls(c,r,&room);
                    if neighbours <4 {
                        removed_count+=1;
                        rolls_to_remove.push((r,c));
                    }
                }         
            } 
        }
        for (r,c ) in rolls_to_remove{
            room[r][c]=FREE;
        }
        result=result +removed_count;
        println!("removed {}",removed_count);
        if removed_count ==0
        {
            break;
        }
    }

    result
}

fn main() {
    println!("AoC 2025 Riddle 4b");
    //let filename = "../../4/test.txt";
    let filename = "../../4/riddle.txt";
    let result = solve(filename);
    println!("Result: {}", result);
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