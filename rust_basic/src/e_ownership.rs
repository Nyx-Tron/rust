pub fn ownership() {
    /* Copy */
    let num = 32;
    let num2 = num; // 因为 i32 实现了 Copy， 所以这个操作将 num 复制到了 num2
    println!("num: {}, num2: {}", num, num2);

    /* Move */
    // 未定义 Copy 类型的 Move 错误
    // let name = String::from("Nyx");
    // let name2 = name;
    // println!("name: {}, name2: {}", name, name2); // 'name' value used after being moved

    // 使用 clone 解决报错
    let name = String::from("Nyx");
    let name2 = name.clone();
    println!("name: {}, name2: {}", name, name2);

    // 传参过程中导致的常见 Move 错误
    // let name = String::from("Nyx");
    // say_hello(name);
    // println!("name: {}", name); // value used after being moved

    // 使用 clone 解决报错
    let name = String::from("Nyx");
    say_hello(name.clone());
    println!("name: {}", name); // value used after being moved
}

fn say_hello(name: String) {
    println!("Hello, {}", name);
}