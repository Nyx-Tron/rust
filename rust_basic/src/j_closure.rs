pub fn closure()
{
    /* 闭包的简单用法 */
    // let sum1 = |a: i32, b: i32| a + b;
    // let sum2 = |a: i32, b: i32| {
    //     a + b
    // };
    // println!("10 + 20 = {}", sum2(10, 20));


    /* 闭包的使用 */
    // let name = String::from("Free");
    // let say = || say_hello(name);
    // delegate(say); // 重复调用两次 say_hello 函数，消耗所有权。
    // 上面三行代码等价于 以下结构体
    // struct My {
    //     name: String,
    // }
    // impl My {
    //     fn new(name: String) -> Self {
    //         Self { name }
    //     }
    //     fn call(&self) {
    //         /*
    //             1. &self = Fn()
    //             2. &mut self = FnMut()
    //             3. self = FnOnce()
    //             FnOnce(FnOnce,FnMut,Fn) --> FnMut(FnMut,Fn) --> Fn(Fn)
    //         */
    //         say_hello(self.name.clone());
    //     }
    // }
    // let my = My::new(name);
    // my.call();
    // my.call(); // 重复调用两次 say_hello 函数，消耗所有权。


    /* move 关键字 */
    // let name = String::from("Free");
    // let say = move || say_hello(name); // 使用 move 指明闭包捕获 name 所有权（这里不加 move 默认获取所有权）
    // delegate(say);

    /* 循环 */
    (0..10).for_each(|i| println!("{}", i)); // 范围.for_each方法(闭包)
}


fn say_hello(name: String){
    println!("Hello, {}", name);
}
fn delegate<F: Fn()>(f: F) {
    f();
    f();
}

