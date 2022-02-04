use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f1 = File::open("hello1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });

    // let f2 = File::open("hello2.txt").unwrap();

    // let f3 = File::open("hello3.txt").expect("Failed to open hello3.txt");

    read_username_from_file();

    read_username_from_file1();

    read_username_from_file2();

    read_username_from_file3();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello4.txt");

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

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello4.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello4.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello5.txt")
}
