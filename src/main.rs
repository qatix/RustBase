use std::io::{BufWriter, stdout};

use ferris_says::say;

use myclass::MyClass;
use myvector::vector_test;
use rust_base::english::farewells::farewells::goodbye;
extern crate rust_base;

mod myclass;
mod myvector;
mod mymap;

mod mydatatype;
mod myfunc;
mod myloop;
mod mystring;
mod mybase;
mod mycond;
mod mystruct;
mod myslice;
mod myenum;
mod mymod;
mod mygeneric;
mod mytrait;
mod myio;
mod myerror;
mod mythread;

fn say_test() {
    let stdout = stdout();
    let message = String::from("hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn main() {
    println!("Hello, rust!");

    let args = std::env::args();
    println!("{:?}", args);

    for arg in args {
        println!("arg:{}", arg);
    }

    // let object = MyClass::new(String::from("rust"),111);
    // object.public_method();

    // vector_test();
    //
    // mymap::map_test();
    //
    // mydatatype::dt_test();
    //
    // myfunc::func_test();
    //
    // myloop::loop_test();
    //
    // mystring::str_test();
    //
    // mybase::base_test();
    //
    // mycond::condition_test();
    //
    // say_test();

    // mystruct::struct_test();

    // myenum::enum_test();

    // mymod::mod_test();

    // mygeneric::generic_test();

    // mytrait::trait_test();

    // myio::read_file();
    // myio::write_file();

    // myerror::error_test();

    // mythread::thread_test();

    // myslice::slice_test();

    let gbye = goodbye();
    println!("bye:{}", gbye);
    println!("ll:{}",rust_base::english::greetings::hello());
    rust_base::nation::govern();
}
