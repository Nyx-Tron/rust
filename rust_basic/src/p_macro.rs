

macro_rules! say_hello {
    () => {
        println!("Hello, marco!");
    };
    ($name:expr) => { // expr：字符串
        // println!("Hello, {$name}!")  is wrong. ❌
        println!("Hello, {}!", $name);
    };
}


macro_rules! nameof {
    ($name:ident) => { // ident：标识符
        stringify!($name) // 不要加分号， 宏返回值
    };
}

macro_rules! create_fn {
    ($name:ident, $body:expr) => {
        fn $name()  -> i32 {
            $body
        }
    };
}


macro_rules! create_struct {
    // &()* ： 将（）中的东西重复 0 到 n 次
    // 宏 忽略空格
    ($name:ident, $($field_name:ident: $field_type:ty),    *) => { // ty：表示数据类型
        #[derive(Debug)]
        struct $name {
            $($field_name: $field_type),* // 用一个 * 号，重复前面（）里的部分
        }
        impl $name {
            fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name),*
                }
            }
        }
    };
}

macro_rules! hashmap {
    ($($key:expr => $value:expr),*  $(,)?) => { // $()? ：表示（）里面的东西可有可无
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

/* 1. 派生宏 */
// #[derive()]

/* 2. 属性宏 */
// 有点像 java 中的 注解

/* 3. 函数宏 */


pub fn rust_macro()
{
    say_hello!(); // 宏调用 必须在 宏定义 后面
    say_hello!("marco");

    println!("struct name is {}", nameof!(Person));

    create_fn!(get_one, 1);
    create_fn!(get_ten, 10);
    println!("get one : {}", get_one());
    println!("get ten : {}", get_ten());

    create_struct!(Student, name: String, age: u8);
    let student = Student::new(String::from("Nyx"), 21);
    println!("{}: {:#?}", nameof!(student), student);

    let map = hashmap!("Nyx"=>"12", "Lzh"=>"21");
    println!("{}: {:#?}", nameof!(map), map);
}


struct Person;
