struct User {
    name: String,
    age: u16,
    city: String,
    sex: u8,
    status: bool,
}

struct Color(u8, u8, u8);

struct Point(f64, f64);

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }

    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

//单元结构体
// 结构体可以值作为一种象征而无需任何成员
struct UnitStruct;

pub fn struct_test() {
    let user1 = User {
        name: String::from("rust"),
        age: 111,
        city: String::from("beijing"),
        sex: 1,
        status: true,
    };
    println!("user.name = {} user.age={},user.city={}", user1.name, user1.age
             , user1.city);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);


    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 area is {}", rect1.area());

    let rect2 = Rectangle { width: 40, height: 20 };
    println!("{}", rect1.wider(&rect2));

    let rect3 = Rectangle::create(11, 22);
    println!("rect3 w={},h={}", rect3.width, rect3.height);
}