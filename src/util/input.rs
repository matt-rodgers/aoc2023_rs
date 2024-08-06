use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn get_lines(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename).context("Couldn't open input file")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();

    if lines.len() == 0 {
        bail!("Got zero lines from file");
    }

    Ok(lines)
}

pub fn get_all(filename: &str) -> Result<String> {
    let mut file = File::open(filename).context("Couldn't open input file")?;
    let mut data = String::new();
    let _ = file
        .read_to_string(&mut data)
        .context("Failed to read file")?;
    Ok(data)
}
