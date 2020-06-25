use ferris_says::say;
use std::io::{stdout,BufWriter};

mod myclass;
use myclass::MyClass;

mod myvector;
use myvector::{vector_test};

mod mymap;

mod mydatatype;
mod myfunc;
mod myloop;
mod mystring;
mod mybase;
mod mycond;

fn say_test(){
   let stdout = stdout();
   let message = String::from("hello fellow Rustaceans!");
   let width = message.chars().count();

   let mut writer = BufWriter::new(stdout.lock());
   say(message.as_bytes(),width,&mut writer).unwrap();
}

fn main() {
    println!("Hello, world!");

   let object = MyClass::new(String::from("rust"),111);
   object.public_method();

   vector_test();

   mymap::map_test();

   mydatatype::dt_test();

   myfunc::func_test();

   myloop::loop_test();

   mystring::str_test();

   mybase::base_test();

   mycond::condition_test();

   say_test();
}
