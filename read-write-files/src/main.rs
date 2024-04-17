use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static LOREM_IPSUM: &str = "Lorem ipsum dolor";

fn main() {
    // Create a path to the desired file
    let path = Path::new("info.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("Não foi possível abrir o arquivo: {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed

    let path_write = Path::new("lorem_ipsum.txt");
    let display = path_write.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file_write = match File::create(&path_write) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file_write) => file_write,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file_write.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}