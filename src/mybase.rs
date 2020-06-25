pub fn base_test(){
    let a = 123;
    println!("a = {}", a);

    let x = 5;
    let x = x * 2;
    println!("The value of x is:{}", x);

    let mut y = 456;
    println!("first y= {}", y);
    y = 678;
    println!("second y = {}", y);

    let s = "1234";
    println!("x = {} len:{}", s, s.len());
}