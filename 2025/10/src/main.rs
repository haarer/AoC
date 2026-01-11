
use regex::Regex; // cargo add regex (for parsing)

fn gen_pairs(count: usize) -> Vec<(usize, usize)> {
    let mut second_idx_start = 1;

    let mut pairs = vec![];

    for first_idx in 0..count - 1 {
        for second_idx in second_idx_start..count {
            pairs.push((first_idx, second_idx));
        }
        second_idx_start += 1;
    }
    pairs
}

fn gen_2hochn_perm(count: usize) -> Vec<Vec<usize>> {
    let mut output: Vec<Vec<usize>> = vec![];
    for i in 0..2_usize.pow(count as u32) {
        let mut bitvec: Vec<usize> = vec![];
        for bit in 0..count {
            let t = (i >> bit) & 0x000001;
            if t > 0 {
                bitvec.push(bit);
            }
        }
        output.push(bitvec);
    }

    output
}
fn xor_bitline(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut result: [bool; 16] = [false; 16];
    for idx in 0..16 {
        result[idx] = a[idx] ^ b[idx];
    }

    result
}
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
    button_logic: [[bool; 16]; 16],
    target_lights: [bool; 16],
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
            .map(|n| n.trim().parse().unwrap())
            .collect();

        Machine {
            lights,
            buttons,
            joltages,
            button_logic: [[false; 16]; 16],
            target_lights: [false; 16],
        }
    }
    fn solve_logic(&mut self) -> usize {
        println!("set logic for machine");
        println!("num of buttons {}", self.buttons.len());
        println!("num of lights {}", self.lights.len());
        let mut buttons_used: Vec<usize> = vec![];

        for button_index in 0..self.buttons.len() {
            for &light_index in &self.buttons[button_index] {
                self.button_logic[button_index][light_index] = true;
                // this button toggles that light we need to light
                // so consider it
                if self.lights[light_index]
                    && buttons_used.iter().find(|&x| *x == button_index) == None
                {
                    //
                }
            }
            buttons_used.push(button_index);
        }
        for i in 0..self.lights.len() {
            self.target_lights[i] = self.lights[i];
        }

        // test combinations of buttons, xor their array lines and compare to target
        let mut results: Vec<Vec<usize>> = vec![];
        let perm = gen_2hochn_perm(buttons_used.len());
        for buttons in perm.iter() {
            //println!("testing buttons {:?}", buttons);
            let mut bitline: [bool; 16] = [false; 16];
            // press buttons and exor result
            for b in buttons {
                bitline = xor_bitline(bitline, self.button_logic[*b]);
            }
            // is this a result?
            if bitline == self.target_lights {
                results.push(buttons.clone());
            }
        }

        results.sort_by(|a, b| a.len().cmp(&b.len()));
        match results.first() {
            Some(btns) => {
                println!("solution {:?}", btns);
                btns.len()
            }
            _ => {
                println!(" no result found");
                0
            }
        }
    }
}

struct Puzzle {
    machines: Vec<Machine>,
    result1: usize,
    result2: i64,
}

impl Puzzle {
    fn new(filename: &str) -> Puzzle {
        let contents: String = std::fs::read_to_string(filename).expect("File not found");
        let mut machines: Vec<Machine> = vec![];
        let mut maxbuttons = 0;
        let mut maxlights = 0;
        let mut result1 = 0;
        for line in contents.lines() {
            let mut m = Machine::new_from_string(line.to_string());
            if m.buttons.len() > maxbuttons {
                maxbuttons = m.buttons.len();
            }
            if m.lights.len() > maxlights {
                maxlights = m.lights.len();
            }
            result1 += m.solve_logic();
            machines.push(m);
        }
        println!("loaded {} machines", machines.len());
        println!("max buttons: {}, max lights: {}", maxbuttons, maxlights);
        Puzzle {
            machines,
            result1: result1,
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
