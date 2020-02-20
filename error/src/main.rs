use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    // Error Handling: use match is the most primitive way, then unwrap/expect/unwrap_or_xxx.
    let f = File::open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });
    // `unwrap` will call `panic!` for us
    let f = File::open("test.txt").unwrap();
    // `expect` will provide out message
    let f = File::open("test.txt").expect("Failed to open test.txt");

    // Propagation: If we want caller to handle the error, we can propagate it with Result enum
    fn read_username_from_file_v1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
    
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut s = String::new();
    
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    // Use `?` operator to automatically return error with `from` function.
    // Note that the return type must be compatible to use `?`.
    fn read_username_from_file_v2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("test.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
    // Use associated function
    fn read_username_final_ver() -> Result<String, io::Error> {
        fs::read_to_string("test.txt") // note that no semicolon here.
    }

}
