use geo::{BoundingRect, Contains, Coord};

use itertools::Itertools; // cargo add itertools (for collect_tuple() )

struct Polygon {
    po: geo::Polygon<f64>,
}
impl Polygon {
    fn new_from_string(contents: String) -> Polygon {
        let mut points: Vec<(i64, i64)> = vec![];
        for line in contents.lines() {
            let (x, y) = line
                .split(',')
                .collect_tuple()
                .expect("failed to parse line");
            points.push((
                x.parse::<i64>().expect("failed to parse to i64"),
                y.parse::<i64>().expect("failed to parse to i64"),
            ));
        }

        let linestr_flt: geo::LineString<f64> =
            points.iter().map(|p| (p.0 as f64, p.1 as f64)).collect();

        let po = geo::Polygon::new(linestr_flt, vec![]);
        Polygon { po }
    }

    fn len(&self) -> usize {
        self.po.exterior().points().count()
    }

    fn check_point_inside(&self, idx_first: usize, idx_second: usize) -> bool {
        // check if the points defined by index are inside the poly
        // unfortunatley the contains check works only with float

        let rect: geo::Rect<f64> = geo::Rect::new(
            (
                self.po.exterior()[idx_first].x as f64,
                self.po.exterior()[idx_first].y as f64,
            ),
            (
                self.po.exterior()[idx_second].x as f64,
                self.po.exterior()[idx_second].y as f64,
            ),
        );

        self.po.contains(&rect)
    }
}

// implement index trait for polygon to allow [] notation access points
use std::ops::Index;
impl Index<usize> for Polygon {
    type Output = Coord<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.po.exterior()[index]
    }
}

fn compute_area(p1: &Coord<f64>, p2: &Coord<f64>) -> f64 {
    // area includes both ends, so +1
    ((p1.x - p2.x).abs() + 1.) * ((p1.y - p2.y).abs() + 1.)
}

struct Puzzle {
    poly: Polygon,
    result1: i64,
    result2: i64,
}

impl Puzzle {
    fn new(filename: &str) -> Puzzle {
        let contents: String = std::fs::read_to_string(filename).expect("File not found");

        //load points
        let poly = Polygon::new_from_string(contents);
        let num_of_points = poly.len();
        println!("loaded {} points", num_of_points);
        println!("means {} tests", num_of_points * (num_of_points - 1) / 2);

        let bbox = &poly.po.bounding_rect().unwrap();
        println!(
            "bbox= ({},{})-({},{})",
            bbox.min().x,
            bbox.min().y,
            bbox.max().x,
            bbox.max().y
        );

        // find max area of point pairs unconstrained (riddle part a)
        let require_area_inside = true;

        let mut second_idx_start = 1;
        let mut max_area = 0.;
        let mut max_p1 = 0;
        let mut max_p2 = 0;

        let mut max_area_strict = 0.;

        for first_idx in 0..poly.len() - 1 {
            for second_idx in second_idx_start..poly.len() {
                let area = compute_area(&poly[first_idx], &poly[second_idx]);

                if require_area_inside && (area > max_area_strict) {
                    // we must test if area is in side of polygon but its not - so skip this area
                    // but do the expensive check only if area is larger
                    if poly.check_point_inside(first_idx, second_idx) {
                        max_area_strict = area;
                        print!(
                            "testing {}({},{}) against {}({},{}) ",
                            first_idx,
                            poly[first_idx].x,
                            poly[first_idx].y,
                            second_idx,
                            poly[second_idx].x,
                            poly[second_idx].y,
                        );
                        println!(" --> inside, area={}", area);
                    } else {
                        continue;
                    }
                }
                if area > max_area {
                    max_area = area;
                    max_p1 = first_idx;
                    max_p2 = second_idx;
                }
            }
            second_idx_start += 1;
        }
        println!(
            "max area is {} from p1({},{}) and p2({},{})",
            max_area, poly[max_p1].x, poly[max_p1].y, poly[max_p2].x, poly[max_p2].y,
        );

        // find max area

        Puzzle {
            poly,
            result1: 0,
            result2: 0,
        }
    }

}

use std::time::Instant;

fn main() {

    println!("AoC 2025 Riddle 9");
    //let filename = "../9/test.txt";
    let filename = "../9/riddle.txt";

    let start = Instant::now();
    let puzzle = Puzzle::new(filename);
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

Your puzzle answer was 4743645488.

The first half of this puzzle is complete! It provides one gold star: *
--- Part Two ---

The Elves just remembered: they can only switch out tiles that are red or green. So, your rectangle can only include red or green tiles.

In your list, every red tile is connected to the red tile before and after it by a straight line of green tiles. The list wraps, so the first red tile is also connected to the last red tile. Tiles that are adjacent in your list will always be on either the same row or the same column.

Using the same example as before, the tiles marked X would be green:

..............
.......#XXX#..
.......X...X..
..#XXXX#...X..
..X........X..
..#XXXXXX#.X..
.........X.X..
.........#X#..
..............

In addition, all of the tiles inside this loop of red and green tiles are also green. So, in this example, these are the green tiles:

..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............

The remaining tiles are never red nor green.

The rectangle you choose still must have red tiles in opposite corners, but any other tiles it includes must now be red or green. This significantly limits your options.

For example, you could make a rectangle out of red and green tiles with an area of 15 between 7,3 and 11,1:

..............
.......OOOOO..
.......OOOOO..
..#XXXXOOOOO..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............

Or, you could make a thin rectangle with an area of 3 between 9,7 and 9,5:

..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXXOXX..
.........OXX..
.........OX#..
..............

The largest rectangle you can make in this example using only red and green tiles has area 24. One way to do this is between 9,5 and 2,3:

..............
.......#XXX#..
.......XXXXX..
..OOOOOOOOXX..
..OOOOOOOOXX..
..OOOOOOOOXX..
.........XXX..
.........#X#..
..............

Using two red tiles as opposite corners, what is the largest area of any rectangle you can make using only red and green tiles?


*/
