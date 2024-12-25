use std::fs::{File, OpenOptions};
use std::io::{self, Write};

pub fn save_to_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}
