pub fn condition_test() {
    let num = 3;
    if num < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }
    println!("b = {}", b);

    let num3 = if a > 0 { 1 } else { -1 };
    println!("num3={}", num3);
}