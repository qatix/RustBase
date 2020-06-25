use std::fs::File;
use std::io;
use std::io::Read;

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn error_test(){
    //way.1
    // panic!("error occured");
    println!("hello rust error");

    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.");
        }
    }

    let f2 = File::open("hello.txt");
    if let Ok(file) = f2 {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }


    // let f3 = File::open("hello.txt").unwrap();
    // let f4 = File::open("hello.txt").expect("Failed to open.");

    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}
