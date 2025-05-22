use std::{fs, io};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = fs::File::open("output.log")?;
    let reader = BufReader::new(file);

    let mut total_lines = 0;
    let mut level_counts_map = HashMap::new();

    let log_levels = vec!["ERROR", "INFO", "WARNING"];

    for line in reader.lines() {
        let line = line?;
        total_lines += 1;

        for level in &log_levels {
            if line.contains(level) {
                level_counts_map
                .entry(level)
                .and_modify(|val| *val += 1)
                .or_insert(1);
            }
        }
    }

    println!("Total lines: {}", total_lines);
    println!("\nLog level counts: ");

    for (key, val) in &level_counts_map {
        println!("- {}: {}", key, val);
    }

    if let Some((key, val)) =
     level_counts_map.iter().max_by_key(|entry| entry.1) {
        println!("\nMost common log level: {} ({} times)", key, val);
     }


    Ok(())
}
