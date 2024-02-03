use std::fs::File;
use std::io::{self, BufReader, BufRead};

fn new_reader(fname: &str) -> io::Result<BufReader<File>> {
    Ok(BufReader::new(File::open(fname)?))
}

pub fn read_matrix(fname: &str) -> io::Result<Vec<Vec<u32>>> {
    let reader = new_reader(fname)?;
    let mut data = Vec::new();
    let mut memb: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let l = match line {
            Ok(s) => s,
            Err(_) => continue,
        };

        if l.trim().is_empty() {
            data.push(memb);
            memb = Vec::new();
            continue;
        }

        memb.push(l.parse::<u32>().unwrap());
    }

    Ok(data)
}

pub fn read_str_pairs(fname: &str) -> io::Result<Vec<(String, String)>> {
    let reader = new_reader(fname)?;
    let mut data: Vec<(String, String)> = Vec::new();

    for line in reader.lines() {
        let l = match line {
            Ok(s) => s,
            Err(_) => continue,
        };

        if l.trim().is_empty() {
            continue;
        }

        let mut splits = l.split_whitespace();
        data.push((
            splits.next().unwrap().to_string(),
            splits.next().unwrap().to_string()
        ));
    }

    Ok(data)
}

pub fn read_lines(fname: &str) -> io::Result<Vec<String>> {
    Ok(
        new_reader(fname)?
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.trim().to_string())
            .collect()
    )
}

pub struct Assignment {
    pub lower: u8,
    pub upper: u8,
}

pub fn read_assignments(fname: &str) -> io::Result<Vec<(Assignment, Assignment)>> {
    let reader = new_reader(fname)?;
    let mut data: Vec<(Assignment, Assignment)> = Vec::new();

    for line in reader.lines() {
        let l = match line {
            Ok(s) => s,
            Err(_) => continue,
        };

        if l.trim().is_empty() {
            continue;
        }

        let mut pair_splits = l.split(',');
        let mut set1_splits = pair_splits.next().unwrap().split('-');
        let mut set2_splits = pair_splits.next().unwrap().split('-');

        data.push((
            Assignment {
                lower: set1_splits.next().unwrap().parse::<u8>().unwrap(),
                upper: set1_splits.next().unwrap().parse::<u8>().unwrap(),
            },
            Assignment {
                lower: set2_splits.next().unwrap().parse::<u8>().unwrap(),
                upper: set2_splits.next().unwrap().parse::<u8>().unwrap(),
            }
        ))
    }

    Ok(data)
}

