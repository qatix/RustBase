enum Book1 {
    Papery,
    Electronic,
}

enum Book2 {
    Papery { index: u32 },
    Electronic { url: String },
}


enum Book {
    Papery { index: u32 },
    Electronic { url: String },
}


pub fn enum_test() {
    // let book1 = Book::Papery;
    // println!("book = {:?}",book1);

    let book2 = Book2::Papery { index: 1001 };

    let book = Book::Papery { index: 1001 };
    let ebook = Book::Electronic { url: String::from("url...") };

    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        }
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
    // println!("{}",ebook);
    match ebook {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        }
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }


    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }

    let opt2: Option<&str> = Option::None;
    match opt2 {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }


    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    let i = 0;
    match i {
        0 => println!("zero"),
        _ => {}
    }

    //if let
    let i = 0;
    if let 0 = i {
        println!("zero");
    }
}