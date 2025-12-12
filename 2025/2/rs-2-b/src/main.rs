
fn solve(filename: &str)-> usize
{
    let contents = std::fs::read_to_string(filename).expect("File not found");
    let mut  ranges: Vec<&str> = Vec::new();
    for line in contents.lines() {
        ranges.extend(line.split(",") ) ;
    }
    let mut count  =0;
    let mut result=0;

    for el in ranges {
        println!( "range: {}",el);
        let Some((s,e))= el.split_once("-") else { panic!("cannot split") };
        let start= s.parse::<usize>().unwrap();
        let end= e.parse::<usize>().unwrap();

       for t in start .. end + 1 {
            let id_str = t.to_string();
            let id_len=id_str.len();

            for div in 1 .. id_len { 
                if id_len % div == 0 {
                    if &id_str[ .. id_len / (id_len / div)].repeat(id_len/div) == &id_str {
                        //println!("divider: {}",div);
                        count+=1;
                        println! ("id :{}",id_str);
                        result+=t;
                        break
                    }
                }
            }
        }
    }
    println!("Tested: {}", count);
    result
}

fn main() {
    println!("AoC 2025 Riddle 2b");
    let filename = "../../2/riddle.txt";
    let result = solve(filename);
    println!("Result: {}", result);
}

