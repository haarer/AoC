
fn count_surrounding_rolls(col:usize,row:usize, v:&Vec<Vec<u8>>)->usize{
    let mut count=0;
    if v[row-1][col-1] == 64{ count+=1;}
    if v[row-0][col-1] == 64{ count+=1;}
    if v[row+1][col-1] == 64{ count+=1;}

    if v[row-1][col-0] == 64{ count+=1;}

    if v[row+1][col-0] == 64{ count+=1;}

    if v[row-1][col+1] == 64{ count+=1;}
    if v[row-0][col+1] == 64{ count+=1;}
    if v[row+1][col+1] == 64{ count+=1;}
  
    count
}


fn solve(filename: &str)-> usize
{
    let contents = std::fs::read_to_string(filename).expect("File not found");
    let mut result:usize= 0;

    let cols=contents.lines().last().unwrap().len();
    let rows =contents.lines().count();
    let mut room= vec![vec![46u8; cols+2];  rows+2];
    let mut row=1;
    for line in contents.lines() {
        room[row]=line.as_bytes().to_vec(); // 64 is occupied 46 is free
        room[row].push(46u8);
        room[row].insert(0,46u8);
        row+=1;      
    }
    
    for r in 1.. rows+1{
        for c in 1..cols+1{
            if room[r][c]==64{
                let neighbours = count_surrounding_rolls(c,r,&room);
                if neighbours <4 {
                    result=result +1;
                }
            }         
        } 
    }
    result
}

fn main() {
    println!("AoC 2025 Riddle 4a");
    //let filename = "../../4/test.txt";
    let filename = "../../4/riddle.txt";
    let result = solve(filename);
    println!("Result: {}", result);
}

/*
--- Day 4: Printing Department ---

You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).

Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.

"Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."

If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.

The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.

For example:

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

The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.

In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):

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

Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

*/