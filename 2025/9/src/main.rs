use itertools::Itertools; // cargo add itertools (for collect_tuple() )

struct Puzzle {
    contents: String,
    result1: i64,
    result2: i64,
}

struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
    fn compute_dist(p1: &Point, p2: &Point) -> i64 {
        ((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) ).isqrt()
    }
    fn compute_area(p1: &Point, p2: &Point) -> i64 {
        // area includes both ends, so +1 
       ( (p1.x - p2.x).abs()+1) * ((p1.y - p2.y).abs() +1)
    }
}
impl Puzzle {
    fn new(filename: &str) -> Puzzle {
        let contents: String = std::fs::read_to_string(filename).expect("File not found");

        //load points
        let mut points: Vec<Point> = vec![];
        for line in contents.lines() {
            let (x, y) = line
                .split(',')
                .collect_tuple()
                .expect("failed to parse line");
            points.push(Point::new(
                x.parse::<i64>().expect("failed to parse to i64"),
                y.parse::<i64>().expect("failed to parse to i64"),
            ));
        }
        let num_of_points=points.len();
        println!("loaded {} points",num_of_points);
        println!("means {} tests",num_of_points*(num_of_points-1)/2);

        let mut second_idx_start = 1;
        let mut max_area=0;
        let mut max_p1=0;
        let mut max_p2=0;

        for first_idx in 0..points.len() - 1 {
            for second_idx in second_idx_start..points.len() {               
                //println!("testing {} against {}",first_idx,second_idx);
                let area=Point::compute_area(&points[first_idx],&points[second_idx]);
                if area > max_area{
                    max_area=area;
                    max_p1=first_idx;
                    max_p2=second_idx;
                }
            }
            second_idx_start += 1;
        }
        println!("max area is {} from p1({},{}) and p2({},{})",max_area,points[max_p1].x,points[max_p1].y,points[max_p2].x,points[max_p2].y,);
        Puzzle {
            contents,
            result1: 0,
            result2: 0,
        }
    }
}

use std::{time::Instant, usize};

fn main() {
    println!("AoC 2025 Riddle 9");
    //let filename = "../9/test.txt";
    let filename = "../9/riddle.txt";

    let start = Instant::now();
    let mut puzzle = Puzzle::new(filename);
    let elapsed = start.elapsed();
    println!("Result1: {}", puzzle.result1);
    println!("Result2: {}", puzzle.result2);
    println!("Millis: {} ms", elapsed.as_millis());
}
/*
--- Day 9: Movie Theater ---

You slide down the firepole in the corner of the playground and land in the North Pole base movie theater!

The movie theater has a big tile floor with an interesting pattern. Elves here are redecorating the theater by switching out some of the square tiles in the big grid they form. Some of the tiles are red; the Elves would like to find the largest rectangle that uses red tiles for two of its opposite corners. They even have a list of where the red tiles are located in the grid (your puzzle input).

For example:

7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3

Showing red tiles as # and other tiles as ., the above arrangement of red tiles would look like this:

..............
.......#...#..
..............
..#....#......
..............
..#......#....
..............
.........#.#..
..............

You can choose any two red tiles as the opposite corners of your rectangle; your goal is to find the largest rectangle possible.

For example, you could make a rectangle (shown as O) with an area of 24 between 2,5 and 9,7:

..............
.......#...#..
..............
..#....#......
..............
..OOOOOOOO....
..OOOOOOOO....
..OOOOOOOO.#..
..............

Or, you could make a rectangle with area 35 between 7,1 and 11,7:

..............
.......OOOOO..
.......OOOOO..
..#....OOOOO..
.......OOOOO..
..#....OOOOO..
.......OOOOO..
.......OOOOO..
..............

You could even make a thin rectangle with an area of only 6 between 7,3 and 2,3:

..............
.......#...#..
..............
..OOOOOO......
..............
..#......#....
..............
.........#.#..
..............

Ultimately, the largest rectangle you can make in this example has area 50. One way to do this is between 2,5 and 11,1:

..............
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..............
.........#.#..
..............

Using two red tiles as opposite corners, what is the largest area of any rectangle you can make?

*/
