use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashMap};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

use bkstring::bktree::{BkTree};


fn part_zero(_lines: Vec<String>) -> io::Result<String> {
    Ok(0.to_string())
}


fn part_one(lines: Vec<String>) -> io::Result<String> {
    let (result_tx, result_rx): (Sender<(Vec<char>, Vec<char>)>, Receiver<(Vec<char>, Vec<char>)>) = channel();

    fn parse_line(tx: Sender<(Vec<char>, Vec<char>)>, s: String) {
        let mut seen: HashMap<char, i16> = HashMap::new();
        let mut twos: Vec<char> = Vec::new();
        let mut threes: Vec<char> = Vec::new();

        for c in s.chars() {
            *seen.entry(c).or_insert(0) += 1;
        }

        for (c, v) in seen.iter() {
            match v {
                2 => { twos.push(*c); },
                3 => { threes.push(*c); },
                _ => {}
            };
        }

        tx.send((twos, threes)).unwrap();
    }

    let line_length = lines.len();
    let mut children = vec![];

    for line in lines {
        let result_tx_clone = result_tx.clone();

        let child = thread::spawn(move || {
            parse_line(result_tx_clone, line);
        });

        children.push(child);
    }


    let mut twos = 0;
    let mut threes = 0;
    for _ in 0..line_length {
        let items = result_rx.recv().unwrap();
        if items.0.len() > 0 {
            twos += 1;
        }

        if items.1.len() > 0 {
            threes += 1;
        }
    }

    Ok((twos * threes).to_string())
}


fn part_two(lines: Vec<String>) -> io::Result<String> {
    fn sameness(first: Vec<char>, second: Vec<char>) -> usize {
        let mut diff = 0;

        for i in 0..first.len() {
            if first[i] != second[i] {
                diff += 1;
            }
        }

        return diff;
    }

    fn correct_string(first: Vec<char>, second: Vec<char>) -> String {
        let mut output: Vec<char> = vec![];

        for i in 0..first.len() {
            if first[i] == second[i] {
                output.push(first[i]);
            }
        }

        let s: String = output.into_iter().collect();

        return s;
    }

    let mut b: BkTree<char> = BkTree::new(Some(sameness));

    let vectorized_lines: Vec<Vec<char>> = lines.iter().map(|s| { s.chars().collect() }).collect();

    b.add_list(vectorized_lines.clone());

    for line in vectorized_lines {
        let result = b.search(line, 1);

        if result.len() > 1 {
            return Ok(correct_string(result[0].clone(), result[1].clone()));
        }
    }

    Ok("No Answer Found".to_string())
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
