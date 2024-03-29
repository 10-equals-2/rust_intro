use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;


/// This function reads a line from stdin and looks to open that file
///
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("My path is {}", args[0]);
    println!("My arguments are {:?} arguments {:?}", args.len()-1, &args[1..]);
    println!("{}", &args[2]);
    let file = File::open(&args[2]);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}