mod rot13;

use rot13::rot13;
use std::io::{Read, Write};
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} filename", args[0]);
        return;
    }

    let ref filename = args[1];
    
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let encrypted = rot13(&contents);

            match File::create(filename) {
                Err(e) => panic!("Error creating file {}: {}", filename, e),
                Ok(mut file) => {

                    match file.write_all(encrypted.as_bytes()) {
                        Err(e) => panic!("Error writing to file {}: {}", filename, e),
                        Ok(_) => println!("File has been encrypted"),
                    }
                },
            }
        },
        Err(e) => panic!("Error opening file {}: {}", filename, e),
    }
}