/* Box, deref */
use std::ops::Deref;

pub fn smart_pointer() {
    let nun = Box::new(10); // heap
    let num = 10; // stack

    let person: Box<dyn Person> = Box::new(Employee::new(String::from("Nyx"), 21));
    println!("{}", person.get_name());

    let name = Box::new(String::from("Nyx"));
    println!("使用*来解引用(Box<String>): {}", *name);

    let person_ref = &person;
    print_name(person_ref.deref());
    print_name(person.deref());
    print_name(&*person);

    let age = Box::new(20);
    let age_ref = &age;
    print_age(age_ref); // 隐式调用 .deref()
}

fn print_age(age: &u8) {
    println!("age is {}", age);
}

fn print_name(person: &dyn Person) {
    println!("person's name is {}", person.get_name());
}

trait Person {
    fn get_name(&self) -> String;
}

struct Employee {
    name: String,
    age: i32,
}

impl Employee {
    fn new(name: String, age: i32) -> Self {
        Self{
            name,
            age,
        }
    }
}

impl Person for Employee {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}


/* Cell + RefCell */
// use std::cell::{Cell, RefCell};
// pub fn smart_pointer() {
//     // Cell
//     // 内部可变性：拷贝实现
//     // let mut name = Cell::new(String::from("Nyx"));
//     //
//     // println!("name is {}", name.take()); // 拷贝，实现内部可变性
//     //
//     // *name.get_mut() = String::from("Nyx-Tron");
//     // println!("name is {}", name.take());
//     //
//     // name.replace(String::from("Nyx-Tron-new"));
//     // println!("name is {}", name.take());
//
//     // RefCell
//     // 内部可变性：在对变量不持有可变借用的情况下，修改变量
//     // 内部可变性：引用实现，效率高
//     let name = RefCell::new(String::from("Nyx"));
//     name.borrow_mut().push_str("-Tron");
//     println!("name is {}", name.borrow());
// }


/* Rc */
// use std::rc::Rc;
// pub fn smart_pointer() {
//     let name = String::from("Nyx");
//     let user = User{
//         name: name.clone(),
//     };
//     let employee = Employee{
//         name,
//     };
//
//     println!("user: {:?}\nemployee: {:#?}", user, employee);
// }
// #[derive(Debug)]
// struct User {
//     name: String,
// }
// #[derive(Debug)]
// struct Employee {
//     name: String,
// }

// 如果我们无法判断user和employee那个生命周期长，使用rc
// pub fn smart_pointer() {
//     let name = Rc::new(String::from("Nyx")); // Rc 使用后不适用 .clone 了，会有混淆
//     // Rc 实现多重所有权
//     let user = User{
//         name: Rc::clone(&name),
//     };
//     let employee = Employee{
//         name: Rc::clone(&name),
//     };
//
//
//     println!("user: {:?}\nemployee: {:#?}", user, employee);
// }
// #[derive(Debug)]
// struct User {
//     name: Rc<String>, // Reference Counted 引用技术
// }
// #[derive(Debug)]
// struct Employee {
//     name: Rc<String>,
// }