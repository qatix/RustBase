fn another_function() {
    println!("this is a function");
}

fn add(x: i32, y: i32) -> i32 {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
    return x + y;
}

pub fn func_test() {
    another_function();
    println!("add(1+2)={}", add(1, 2));

    fn five() -> i32 {
        5
    }
    println!("five()={}", five());
}