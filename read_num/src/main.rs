use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Result<File, std::io::Error>
    let s = std::fs::read_to_string("foo")?;
    // Result<usize, std::num::ParseIntError>
    let num: usize = s.trim().parse()?;
    println!("file contains number {}", num);
    Ok(())
}
