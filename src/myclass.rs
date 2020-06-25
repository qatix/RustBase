pub struct MyClass {
    name: String,
    age: i32,
}

impl MyClass {
    pub fn new(_name: String, _age: i32) -> MyClass {
        MyClass {
            name: _name,
            age: _age,
        }
    }

    pub fn public_method(&self){
        println!("from public method");
        self.private_method();
        self.object_print();
    }

    fn object_print(&self){
        println!("name:{} age:{}",self.name,self.age);
    }

    fn private_method(&self){
        println!("from private method");
    }
}