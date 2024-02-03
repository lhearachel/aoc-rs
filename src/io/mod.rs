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

