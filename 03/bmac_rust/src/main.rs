use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashMap};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;


fn parse_item(s: &String, start: usize, end: usize) -> usize {
    let ret: usize = s[start + 1..end].trim().parse().unwrap();

    return ret;
}


fn process_line(s: String) -> (usize, usize, usize, usize, usize) {
    let hashtag = s.find("#").unwrap();
    let at_symbol = s.find("@").unwrap();
    let comma = s.find(",").unwrap();
    let colon = s.find(":").unwrap();
    let x_symbol = s.find("x").unwrap();
    let end = s.len();

    let id = parse_item(&s, hashtag, at_symbol);
    let x = parse_item(&s, at_symbol, comma);
    let y = parse_item(&s, comma, colon);
    let width = parse_item(&s, colon, x_symbol);
    let height = parse_item(&s, x_symbol, end);

    return (id, x, y, width, height);
}


fn part_zero(_lines: Vec<String>) -> io::Result<String> {
    Ok(0.to_string())
}


fn part_one(lines: Vec<String>) -> io::Result<String> {
    let mut fabric = [[0; 1_000]; 1_000];

    for line in lines {
        let (id, x, y, width, height) = process_line(line);

        let mut area = 0;

        for i in x..x + width {
            for j in y..y + height {
                fabric[i][j] += 1;
                area += 1;
            }
        }

        println!("area: {}", area);
    }

    let mut area = 0;

    for i in 0..1_000 {
        for j in 0..1_000 {
            if fabric[i][j] > 1 {
                area += 1;
            }
        }
    }

    Ok(area.to_string())
}


fn part_two(lines: Vec<String>) -> io::Result<String> {
    let mut fabric = [[0; 1_000]; 1_000];

    for line in lines.clone() {
        let (id, x, y, width, height) = process_line(line);

        for i in x..x + width {
            for j in y..y + height {
                fabric[i][j] += 1;
            }
        }
    }

    for line in lines {
        let (id, x, y, width, height) = process_line(line);

        let mut fail = false;

        for i in x..x + width {
            for j in y..y + height {
                if fabric[i][j] > 1 {
                    fail = true;
                }
            }
        }

        if !fail {
            return Ok(id.to_string());
        }
    }

    Ok(0.to_string())
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

    let freq = func(f.lines().map(|l| l.unwrap()).collect())?;

    println!("result: {}", freq);

    Ok(())
}
