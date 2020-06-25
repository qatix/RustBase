fn base_test() {
    let s1 = String::from("hellorust");
    let s2 = s1;
    //println!("s1 = {},s2 = {}",s1,s2); s1 已经失效 value borrowed here after move
    println!("s2 = {}", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {},s4 = {}", s3, s4);
}

fn ref_test() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("ref s1 = {},s2 = {}", s1, s2);

    let s3 = s1;//ownership of s1 move to s3
    //println!("s2 = {}",s2); error
    println!("ref s3 = {}", s3)
}


fn str_length(s: &String) -> usize {
    s.len()
}

fn str_add() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("str_add: s3 = {}", s3);
}

fn utf8_test() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    println!("hello = {}", hello);
}

fn utf8_str_len(){
    let s = "hello你好";
    let len = s.chars().count();
    println!("utf8_str_len:{}",len);
}

pub fn str_test() {
    base_test();

    let ss = String::from("calstr_lenth");
    let len = str_length(&ss);
    println!("The length of '{}' is {}.", ss, len);

    let mut ms11 = String::from("hello");
    let ms21 = &mut ms11;
    ms21.push_str("rust");
    println!("ms:{}", ms21);

    ref_test();

    str_add();

    utf8_str_len();


    let s = String::from("EN中文");
    println!("{} len:{}",s, s.len());
    let sub = &s[0..2];
    println!("{}", sub);
    // let sub = &s[0..3];
    // println!("{}", sub);
    /*
    thread 'main' panicked at 'byte index 3 is not a char boundary; it is inside '中' (bytes 2..5) of `EN中文`', /rustc/c7087fe00d2ba919df1d813c040a5d47e43b0fe7/src/libcore/str/mod.rs:1920:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
   **/
}