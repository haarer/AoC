use itertools::Itertools; // cargo add itertools (for collect_tuple() )
use regex::Regex; // cargo add regex (for parsing)

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn new_from_string(contents: String) -> Machine {
        let rex = Regex::new(r"\[([.#]+)\]\s+(.+\))\s+\{(.+)\}").unwrap();
        let captures = rex.captures(&contents).unwrap();
        //println!("Lights: {:?}", captures[1].chars().collect::<Vec<char>>());
        //println!("Buttons: {:?}", &captures[2]);
        //println!("Joltages: {:?}", &captures[3]);

        let lights: Vec<bool> = captures[1].chars().map(|c| c == '#').collect();

        let buttons: Vec<Vec<usize>> = captures[2]
            .split(" ")
            .map(|s| {
                let s_trimmed = s.trim_matches(|c| c == ' ' || c == '(' || c == ')');
                s_trimmed.split(',').map(|n| n.parse().unwrap()).collect()
            })
            .collect();

        let joltages: Vec<usize> = captures[3]
            .split(",")
            .map(|n| n.trim_matches(|c| c == ' ' || c == ',').parse().unwrap())
            .collect();

        Machine {
            lights,
            buttons,
            joltages,
        }
    }
}

struct Puzzle {
    machines: Vec<Machine>,
    result1: i64,
    result2: i64,
}

impl Puzzle {
    fn new(filename: &str) -> Puzzle {
        let contents: String = std::fs::read_to_string(filename).expect("File not found");
        let mut machines: Vec<Machine> = vec![];
        for line in contents.lines() {
            machines.push(Machine::new_from_string(line.to_string()));
        }
        Puzzle {
            machines,
            result1: 0,
            result2: 0,
        }
    }
}

use std::time::Instant;

fn main() {
    println!("AoC 2025 Riddle 10");
    //let filename = "../10/test.txt";
    let filename = "../10/riddle.txt";

    let start = Instant::now();
    let puzzle = Puzzle::new(filename);
    let elapsed = start.elapsed();
    println!("Result1: {}", puzzle.result1);
    println!("Result2: {}", puzzle.result2);
    println!("Millis: {} ms", elapsed.as_millis());
}
/*
--- Day 10: Factory ---

Just across the hall, you find a large factory. Fortunately, the Elves here have plenty of time to decorate. Unfortunately, it's because the factory machines are all offline, and none of the Elves can figure out the initialization procedure.

The Elves do have the manual for the machines, but the section detailing the initialization procedure was eaten by a Shiba Inu. All that remains of the manual are some indicator light diagrams, button wiring schematics, and joltage requirements for each machine.

For example:

[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

The manual describes one machine per line. Each line contains a single indicator light diagram in [square brackets], one or more button wiring schematics in (parentheses), and joltage requirements in {curly braces}.

To start a machine, its indicator lights must match those shown in the diagram, where . means off and # means on. The machine has the number of indicator lights shown, but its indicator lights are all initially off.

So, an indicator light diagram like [.##.] means that the machine has four indicator lights which are initially off and that the goal is to simultaneously configure the first light to be off, the second light to be on, the third to be on, and the fourth to be off.

You can toggle the state of indicator lights by pushing any of the listed buttons. Each button lists which indicator lights it toggles, where 0 means the first light, 1 means the second light, and so on. When you push a button, each listed indicator light either turns on (if it was off) or turns off (if it was on). You have to push each button an integer number of times; there's no such thing as "0.5 presses" (nor can you push a button a negative number of times).

So, a button wiring schematic like (0,3,4) means that each time you push that button, the first, fourth, and fifth indicator lights would all toggle between on and off. If the indicator lights were [#.....], pushing the button would change them to be [...##.] instead.

Because none of the machines are running, the joltage requirements are irrelevant and can be safely ignored.

You can push each button as many times as you like. However, to save on time, you will need to determine the fewest total presses required to correctly configure all indicator lights for all machines in your list.

There are a few ways to correctly configure the first machine:

[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    You could press the first three buttons once each, a total of 3 button presses.
    You could press (1,3) once, (2,3) once, and (0,1) twice, a total of 4 button presses.
    You could press all of the buttons except (1,3) once each, a total of 5 button presses.

However, the fewest button presses required is 2. One way to do this is by pressing the last two buttons ((0,2) and (0,1)) once each.

The second machine can be configured with as few as 3 button presses:

[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}

One way to achieve this is by pressing the last three buttons ((0,4), (0,1,2), and (1,2,3,4)) once each.

The third machine has a total of six indicator lights that need to be configured correctly:

[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

The fewest presses required to correctly configure it is 2; one way to do this is by pressing buttons (0,3,4) and (0,1,2,4,5) once each.

So, the fewest button presses required to correctly configure the indicator lights on all of the machines is 2 + 3 + 2 = 7.

Analyze each machine's indicator light diagram and button wiring schematics. What is the fewest button presses required to correctly configure the indicator lights on all of the machines?

*/
