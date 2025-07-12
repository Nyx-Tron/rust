pub fn complex_types() {
    // 元组
    let tuple = ("Nyx", 21);
    println!("Hello {:?}", tuple);
    println!("{} is {} years old", tuple.0, tuple.1);

    // 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    // 切片
    let slice = &arr; // 全量
    let slice_1 = &arr[1..]; // [2, 3, 4, 5]
    let slice_13 = &arr[1..3]; // [2, 3], 不包含3
    let slice_13_include = &arr[1..=3]; // [2, 3, 4], 包含3
    println!("slice = {:?}", slice);
    println!("slice_1 = {:?}", slice_1);
    println!("slice_13 = {:?}", slice_13);
    println!("slice_13_include = {:?}", slice_13_include);
    // 字符串 - create string
    let s: String = String::from("hello");
    let s1 = "hello".to_string();
    let s2 = "hello".to_owned();

    // 字符串切片
    let s_slice: &str = "Nyx";

    // 结构体 + 枚举
    let user = User {
    id: String::from("1"),
    name: String::from("Nyx"),
    gender: Gender::MALE,
    };
    println!("user = {:?}", user);


    // 给结构体定义方法
    let user = User::new(
        String::from("1"),
        String::from("Nyx"),
        Gender::MALE,
    );

    user.print(); // == User::print(&user);
}


// 结构体
#[derive(Debug)]  // without it , User cannot be formatted using: {:?}
struct User{
    id: String,
    name: String,
    gender: Gender,
}

// 枚举
#[derive(Debug)]
enum Gender {
    MALE,
    FEMALE,
    UNKNOWN,
}

// 为结构体实现方法
impl User {
    // 关联方法
    fn new(id: String, name: String, gender: Gender) -> Self { // Self == User
        Self { id, name, gender }
    }

    // 实例方法
    fn print(&self) {
        println!("{:?}", self);
    }
}

