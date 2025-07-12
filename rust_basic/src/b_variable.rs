pub fn variable() {
    /* 变量不可变 */
    // let name = String::from("Nyx");
    // // name.push_str("2"); // Cannot borrow as mutable
    // println!("Hello, {}", name);

    let mut name = String::from("Nyx");
    name.push_str("2");
    println!("Hello, {}", name);

    /* 变量遮蔽 */
    let name = String::from("Nyx");
    let name = 2025;
    println!("Hello, {}", name);

    let name = String::from("Nyx");
    {
        let name = 2025;
        println!("Hello, {}", name);
    }
    println!("Hello, {}", name);

    /* 常量 */
    const MAX: u32 = 100_000;
    println!("{}", MAX);
}
