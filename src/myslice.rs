pub fn slice_test(){

    let s = String::from("boardcast");
    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{} = {} + {}",s,part1,part2);

    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }


    //String to &str
    let s1 = String::from("Rust");
    let s2 = &s1[..];
    println!("s1={} s2={}",s1,s2);

    //String 类型是 Rust 标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作。String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性。
    //
    // String 和 str 都支持切片，切片的结果是 &str 类型的数据。
    //
    // 注意：切片结果必须是引用类型，但开发者必须自己明示这一点:
    //
    // let slice = &s[0..3];
}