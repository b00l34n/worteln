use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/**
 * Opens the dictionary file at a the path.
 *
 * @p: path to the dictionary file as a string slice 
 * 
 * @return: File type of the dictionary 
 */
pub fn open_file(p: &str) -> File {
    let path = Path::new(p);
    let display = path.display();

    match File::open(&path) {
        Err(reason) => panic!("couldn't open {}: {}", display, reason),
        Ok(file) => file,
    };
}

/**
 * opens and reads the dictionary file 
 */
pub fn read_file(mut f: File) -> String {
   
    // This is where the dictionary is going to be dumped in.
    let mut buffer = String::new();

    match f.read_to_string(&mut buffer) {
        Err(reason) => panic!("couldn't read file: {}", reason),
        Ok(..) => buffer,
    }
}

