use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let mut file = File::create("testing.txt")?;
    file.write_all(b"hellow world")?;
    Ok(())
}