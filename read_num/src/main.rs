use std::error::Error;

fn read_num() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("numfile")?;
    let num: usize = s.trim().parse()?;
    println!("File contains number {}", num);
    Ok(())
}

fn main() {
    if let Err(e) = read_num() {
        eprintln!("Error while reading 'numfile': {:?}", e);
    }
}
