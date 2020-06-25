trait Comparable {
    fn compare(&self, object: &Self) -> i8;
}

impl Comparable for f64 {
    fn compare(&self, object: &f64) -> i8 {
        if &self > &object { 1 } else if &self == &object { 0 } else { -1 }
    }
}

fn max<T: Comparable>(array: &[T]) -> &T {
    let mut max_idx = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_idx]) > 0 {
            max_idx = i;
        }
        i += 1;
    }
    return &array[max_idx];
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


pub fn generic_test() {
    let arr = [1.0, 66.1, 3.1, 93.3, 22.1, 3.3, 4.4];
    println!("max in arr = {}", max(&arr));

    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());

    let p2 = Point { x: 1.1, y: 2.1 };
    println!("p.x = {}", p2.x());
}