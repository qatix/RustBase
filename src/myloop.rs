pub fn loop_test() {
    //1.while
    let mut i = 1;
    while i != 4 {
        println!("while ={}", i);
        i += 1;
    }

    //2.for
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("for1 {}", i);
    }
    for i in 0..5 {
        println!("for2 a[{}] = {}", i, a[i]);
    }

    //3.loop
    let s = ['R', 'U', 'S', 'T'];
    let mut j = 0;
    loop {
        if j >= s.len(){
            break;
        }
        let ch = s[j];
        if ch == '0' {
            break;
        }
        println!("\'{}\'", ch);
        j += 1;
    }

    println!("Exit");
}