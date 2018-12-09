use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;


fn part_zero(_f_buff: Vec<i64>, _start_freq: i64) -> io::Result<i64> {
    Ok(0)
}


fn part_one(lines: Vec<i64>, start_freq: i64) -> io::Result<i64> {
    let mut freq = start_freq;

    for line in lines {
        let adder = line;
        freq += adder;
    }

    Ok(freq)
}


fn part_two(lines: Vec<i64>, start_freq: i64) -> io::Result<i64> {
    let mut seen: HashSet<i64> = HashSet::new();
    seen.insert(start_freq);

    let line_copy = &lines.clone();

    fn part_two_loop(lines: Vec<i64>, start_freq: i64, seen: &mut HashSet<i64>) -> io::Result<i64> {
        let mut freq = start_freq;
        let line_copy = &lines.clone();

        let mut i = 0;

        for line in lines {
            i += 1;
            let adder = line;
            freq += adder;

            if !seen.insert(freq) {
                return Ok(freq);
            }
        }

        return part_two_loop(line_copy.to_vec(), freq, seen);
    }

    // let mut f_copy = BufReader::
    return part_two_loop(line_copy.to_vec(), start_freq, &mut seen);
}


fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();

    let func = match &args[1] as &str {
        "1" => part_one,
        "2" => part_two,
        _ => part_zero
    };

    let filename = &args[2];

    let file = File::open(filename)?;
    let f = BufReader::new(file);

    let freq = func(f.lines().map(|l| l.unwrap().parse::<i64>().unwrap()).collect(), 0)?;

    println!("end frequency: {}", freq);

    Ok(())
}
