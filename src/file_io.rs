use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const PATH_TO_DIC: &str = "./misc/de_DE.dic";

/**
 *
 */
fn open_dic_file() -> File {
    let path = Path::new(PATH_TO_DIC);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(reason) => panic!("couldn't open {}: {}", display, reason),
        Ok(file) => return file,
    };
}

/**
 *
 */
pub fn read_dic_file() -> String {
   
    // If opening the file falses, it let's the programm end with a panic 
    // The file also gets closed when getting out of scope
    let dic_file : File = open_dic_file();
   
    // This is where the dictionary is going to be dumped in.
    let mut buffer = String::new();

    dic_file.read_to_string(&mut buffer)?;
    Ok(())
        
}

