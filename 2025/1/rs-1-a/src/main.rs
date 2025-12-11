fn solve(filename: &str)-> usize
{
    let contents = std::fs::read_to_string(filename).expect("File not found");
    let mut wheel = 50;
    let mut zerocount = 0;
    for line in contents.lines() {
        let dir = &line[0..1];
        let step: usize = line[1..].trim().parse().expect("Invalid number");
        match dir {
            "R" => {
                wheel = (wheel + step) % 100;
            }
            "L" => {
                wheel = (wheel +100 - (step % 100)) % 100;
            }
            _ => panic!("Invalid dir"),
        }
        if wheel == 0 {
            zerocount += 1;
        }
    }
    zerocount
}

fn main() {
    println!("AoC 2025 Riddle 1a");
    let filename = "../../1/riddle.txt";
    let result = solve(filename);
    println!("Result: {}", result);
}

