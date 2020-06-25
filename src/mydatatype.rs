pub fn dt_test() {
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x={} y={}", x, y);

    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余
    println!("sum={} diff={} product={} quotient={} remainder={}", sum, difference, product, quotient, remainder);

    // tup
    let tup: (i32, f64, u8) = (500, 6.4, 11);
    let (x1, y1, z1) = tup;
    println!("x1={} y1={} z1={}", x1, y1, z1);

    // arr
    let arr = [1, 2, 3, 4, 5];
    let str_arr = ["January", "February", "March"];
    let c_arr: [i32; 5] = [3, 4, 5, 6, 7];
    let d_arr = [3; 5]; // equal to let d = [3,3,3,3,3]
    println!("arr={:?}", arr);
    println!("str_arr={:?}", str_arr);
    println!("c_arr={:?}", c_arr);
    println!("d_arr={:?}", d_arr);

    let first = arr[0];
    //err arr[0] = 444
    println!("first={}", first);

    let mut marr = [4, 5, 6];
    marr[0] = 14;
    println!("marr={:?}", marr);
}