pub fn vector_test() {
    let vector: Vec<i32> = Vec::new();
    let vec2 = vec![1, 2, 4, 8];

    let mut vec3 = vec2;
    vec3.push(16);
    vec3.push(32);
    vec3.push(64);
    println!("vec:{:?}", vec3);

    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("after append:{:?}", v1);

    for i in &v1{
        println!("item in v1:{}",i);
    }

    let mut v3 = vec![100, 32, 57];
    println!("before add:{:?}", v3);
    for i in &mut v3 {
        *i += 50;
    }
    println!("after add:{:?}", v3);
}