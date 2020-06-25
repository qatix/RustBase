use std::collections::HashMap;

pub fn map_test(){

    let mut map = HashMap::new();
    map.insert(1,"abc");
    map.insert(3,"cap");

    if let Some(x) = map.get_mut(&1){
        *x = "b"
    }
    println!("map:{:?}",map);


    let mut map2 = HashMap::new();

    map2.insert("color", "red");
    map2.insert("size", "10 m^2");


    for p in map2.iter() {
        println!("{:?}", p);
    }
    map2.entry("color").or_insert("black");
    map2.entry("coc").or_insert("not_exist");
    println!("map2:{:?}",map2);

    println!("{}", map2.get("color").unwrap());
}