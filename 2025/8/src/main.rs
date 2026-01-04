use itertools::Itertools; // cargo add itertools (for collect_tuple() )

struct Puzzle {
    contents: String,
    result1: usize,
    result2: i64,
}

struct Distance {
    first_point_idx: usize,
    second_point_idx: usize,
    distance: i64,
}
impl Distance {
    fn new(first_id: usize, second_id: usize, dist: i64) -> Distance {
        Distance {
            first_point_idx: first_id,
            second_point_idx: second_id,
            distance: dist,
        }
    }
    fn compute_dist(x1: i64, y1: i64, z1: i64, x2: i64, y2: i64, z2: i64) -> i64 {
        ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)).isqrt()
    }
}

struct Point {
    x: i64,
    y: i64,
    z: i64,
    in_circuit: Option<usize>,
}
impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point {
            x,
            y,
            z,
            in_circuit: None,
        }
    }
}
impl Puzzle {
    fn new(filename: &str) -> Puzzle {
        let contents: String = std::fs::read_to_string(filename).expect("File not found");

        //load points
        let mut points: Vec<Point> = vec![];
        for line in contents.lines() {
            let (x, y, z) = line
                .split(',')
                .collect_tuple()
                .expect("failed to parse line");
            points.push(Point::new(
                x.parse::<i64>().expect("failed to parse to i64"),
                y.parse::<i64>().expect("failed to parse to i64"),
                z.parse::<i64>().expect("failed to parse to i64"),
            ));
        }

        //compute distances of points ( only half of the matrix omitting diagonal)
        let mut distances: Vec<Distance> = vec![];
        let mut second_idx_start = 1;

        for first_idx in 0..points.len() - 1 {
            for second_idx in second_idx_start..points.len() {               
                //println!("testing {} against {}",first_idx,second_idx);
                let dist = Distance::new(
                    first_idx,
                    second_idx,
                    Distance::compute_dist(
                        points[first_idx].x,
                        points[first_idx].y,
                        points[first_idx].z,
                        points[second_idx].x,
                        points[second_idx].y,
                        points[second_idx].z,
                    ),
                );
                distances.push(dist);
            }
            second_idx_start += 1;
        }

        // sort distances
        distances.sort_by(|a, b| a.distance.cmp(&b.distance));

        // connect nearest points to "circuits"
        let mut circuits: Vec<Vec<usize>> = vec![];
        let mut circuit_count = 0;
        let mut conn_count = 0;
        let mut last_conn: Distance=Distance::new(0,0 ,0);
        for dist in distances {
            //check if points to be connected are not connected somehow, then make a new circuit
            if    points[dist.first_point_idx].in_circuit  == None
               && points[dist.second_point_idx].in_circuit == None
            {
                circuits.push(vec![dist.first_point_idx, dist.second_point_idx]); // add to new circuit 
                points[dist.first_point_idx].in_circuit  = Some(circuit_count); // refer point to circuit
                points[dist.second_point_idx].in_circuit = Some(circuit_count); // s.o.
                println!("case 1 conn {} and {} to new circuit {}",dist.first_point_idx,dist.second_point_idx,circuit_count);
                circuit_count += 1;
                conn_count += 1;
                last_conn=dist;
            } else if  points[dist.first_point_idx].in_circuit  != None
                    && points[dist.second_point_idx].in_circuit == None
            {
                // first point is already connected, second not. connect second to firsts circuit
                points[dist.second_point_idx].in_circuit = points[dist.first_point_idx].in_circuit; // refer point also to same circuit
                circuits[points[dist.first_point_idx].in_circuit.unwrap()]
                    .push(dist.second_point_idx); // add second point to circuit of first
                println!("case 2 conn {} and {} to existing circuit {}",dist.first_point_idx,dist.second_point_idx,points[dist.first_point_idx].in_circuit.unwrap());

                conn_count += 1;
                last_conn=dist;
            } else if  points[dist.first_point_idx].in_circuit  == None
                    && points[dist.second_point_idx].in_circuit != None
            {
                // second point is already connected, first not. connect first to seconds circuit
                points[dist.first_point_idx].in_circuit = points[dist.second_point_idx].in_circuit; // refer point also to same circuit
                circuits[points[dist.second_point_idx].in_circuit.unwrap()]
                    .push(dist.first_point_idx); // add first point to circuit of second
                println!("case 3 conn {} and {} to existing circuit {}",dist.first_point_idx,dist.second_point_idx,points[dist.second_point_idx].in_circuit.unwrap());
                conn_count += 1;
                last_conn=dist;
            }else if   points[dist.first_point_idx].in_circuit  != None
                    && points[dist.second_point_idx].in_circuit != None
                    && points[dist.first_point_idx].in_circuit != points[dist.second_point_idx].in_circuit {
                // both points are conneced to different circuits. join circuits to one
                println!(
                    "case 4 {} and {} already connected to different circuits {} and {}",
                    dist.first_point_idx,
                    dist.second_point_idx,
                    points[dist.first_point_idx].in_circuit.unwrap(),
                    points[dist.second_point_idx].in_circuit.unwrap());
                let mut points_to_move = circuits[points[dist.second_point_idx].in_circuit.unwrap()].clone();
                circuits[points[dist.second_point_idx].in_circuit.unwrap()]=vec![];
                // set circuit of moved points
                for p in points_to_move.iter(){
                    points[*p].in_circuit=points[dist.first_point_idx].in_circuit;
                    
                }
                // put moved points into other circuit
                circuits[points[dist.first_point_idx].in_circuit.unwrap()].append(&mut points_to_move);       
                conn_count += 1;
                last_conn=dist;
                }else {
                println!(
                    "case 5 {} and {} already connected to same circuits {} and {}",
                    dist.first_point_idx,
                    dist.second_point_idx,
                    points[dist.first_point_idx].in_circuit.unwrap(),
                    points[dist.second_point_idx].in_circuit.unwrap());
                conn_count += 1; // this is stupid but necessary to comply.
                                 // it seems to count as a connection but does not really connect any thing.
            }

            // else do nothing
            // verfummelt um 8b zu lösen
            //if conn_count == 1000 {
            //    break;
            //}
        }

        for c in circuits.iter().enumerate(){
            if c.1.len() > 0 {
            println!("circuit {} with {}",c.0,c.1.iter().map(|x|x.to_string()).join(","));
            }
        }
        println!("last connection was {} to {}",last_conn.first_point_idx,last_conn.second_point_idx);
        println!("last connection x1 {} to  x2 {}",points[last_conn.first_point_idx].x,points[last_conn.second_point_idx].x);


        // sort circuits by size
        circuits.sort_by(|a, b| b.len().cmp(&a.len()));


        Puzzle {
            contents,
            result1: circuits[0].len()*circuits[1].len()*circuits[2].len(),
            result2: points[last_conn.first_point_idx].x * points[last_conn.second_point_idx].x,
        }
    }

    fn solve1(&mut self) {}
}

use std::{time::Instant, usize};

fn main() {
    println!("AoC 2025 Riddle 8");
    //let filename = "../8/test.txt";
    let filename = "../8/riddle.txt";
    let mut puzzle = Puzzle::new(filename);

    let start = Instant::now();
    puzzle.solve1();
    let elapsed = start.elapsed();
    println!("Result1: {}", puzzle.result1);
    println!("Result2: {}", puzzle.result2);
    println!("Millis: {} ms", elapsed.as_millis());
}
/*
--- Day 8: Playground ---

Equipped with a new understanding of teleporter maintenance, you confidently step onto the repaired teleporter pad.

You rematerialize on an unfamiliar teleporter pad and find yourself in a vast underground space which contains a giant playground!

Across the playground, a group of Elves are working on setting up an ambitious Christmas decoration project. Through careful rigging, they have suspended a large number of small electrical junction boxes.

Their plan is to connect the junction boxes with long strings of lights. Most of the junction boxes don't provide electricity; however, when two junction boxes are connected by a string of lights, electricity can pass between those two junction boxes.

The Elves are trying to figure out which junction boxes to connect so that electricity can reach every junction box. They even have a list of all of the junction boxes' positions in 3D space (your puzzle input).

For example:

162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689

This list describes the position of 20 junction boxes, one per line. Each position is given as X,Y,Z coordinates. So, the first junction box in the list is at X=162, Y=817, Z=812.

To save on string lights, the Elves would like to focus on connecting pairs of junction boxes that are as close together as possible according to straight-line distance. In this example, the two junction boxes which are closest together are 162,817,812 and 425,690,689.

By connecting these two junction boxes together, because electricity can flow between them, they become part of the same circuit. After connecting them, there is a single circuit which contains two junction boxes, and the remaining 18 junction boxes remain in their own individual circuits.

Now, the two junction boxes which are closest together but aren't already directly connected are 162,817,812 and 431,825,988. After connecting them, since 162,817,812 is already connected to another junction box, there is now a single circuit which contains three junction boxes and an additional 17 circuits which contain one junction box each.

The next two junction boxes to connect are 906,360,560 and 805,96,715. After connecting them, there is a circuit containing 3 junction boxes, a circuit containing 2 junction boxes, and 15 circuits which contain one junction box each.

The next two junction boxes are 431,825,988 and 425,690,689. Because these two junction boxes were already in the same circuit, nothing happens!

This process continues for a while, and the Elves are concerned that they don't have enough extension cables for all these circuits. They would like to know how big the circuits will be.

After making the ten shortest connections, there are 11 circuits: one circuit which contains 5 junction boxes, one circuit which contains 4 junction boxes, two circuits which contain 2 junction boxes each, and seven circuits which each contain a single junction box. Multiplying together the sizes of the three largest circuits (5, 4, and one of the circuits of size 2) produces 40.

Your list contains many junction boxes; connect together the 1000 pairs of junction boxes which are closest together. Afterward, what do you get if you multiply together the sizes of the three largest circuits?



--- Part Two ---

The Elves were right; they definitely don't have enough extension cables. You'll need to keep connecting junction boxes together until they're all in one large circuit.

Continuing the above example, the first connection which causes all of the junction boxes to form a single circuit is between the junction boxes at 216,146,977 and 117,168,530. The Elves need to know how far those junction boxes are from the wall so they can pick the right extension cable; multiplying the X coordinates of those two junction boxes (216 and 117) produces 25272.

Continue connecting the closest unconnected pairs of junction boxes together until they're all in the same circuit. What do you get if you multiply together the X coordinates of the last two junction boxes you need to connect?


*/
