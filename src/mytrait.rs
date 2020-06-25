use std::os::macos::raw::time_t;

struct Person {
    name: String,
    age: u8,
}

// trait Descriptive {
//     fn describe(&self) -> String;
// }

trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("person:{} {}", self.name, self.age)
    }
}

fn output(object: impl Descriptive) {
    println!("{}", object.describe());
}

fn output2<T: Descriptive>(object: T) {
    println!("{}", object.describe());
}

fn output_two<T: Descriptive>(arg1: T, arg2: T) {
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}

//其它用法：
// fn notify(item: impl Summary + Display)
// fn notify<T: Summary + Display>(item: T)
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U)

pub fn trait_test() {
    let person = Person {
        name: String::from("rust"),
        age: 11,
    };
    println!("person:{}", person.describe());
    let person2 = Person {
        name: String::from("rust1"),
        age: 12,
    };
    output(person2);

    let person3 = Person {
        name: String::from("rust2"),
        age: 14,
    };
    output2(person3);
}